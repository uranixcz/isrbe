use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use isrbe::{catch_mysql_err, match_id, ERROR_PAGE, TransformType, get_transform_types, get_quantities};
use isrbe::locations::ResLocationResolved;
use isrbe::locations::transport::{get_res_amount_at_location, update_res_amount_at_location};
use std::borrow::Cow;

#[derive(Serialize)]
struct TransformContext<'a> {
    types: &'a Vec<TransformType>,
    transform: Option<Transform<'a>>,
    locations: Vec<ResLocationResolved<'a>>,
}

#[derive(Serialize, Debug)]
struct Transform<'a> {
    id: u64,
    type_id: u64,
    type_name: &'a str,
    refer: String,
    lines: Vec<TransformLine<'a>>,
}
impl<'a> FromRow for Transform<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, type_id, refer) = deconstruct.unwrap();
            Ok(Transform {
                id,
                type_id,
                type_name: "",
                refer,
                lines: Vec::new(),
            })
        }
    }
}
#[derive(Serialize, Debug)]
struct TransformLine<'a> {
    id: u64,
    amount: f64,
    location: ResLocationResolved<'a>,
}
impl<'a> FromRow for TransformLine<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, amount, loc_id, loc_amount, lat, lon, radius, unit_id, res_name) = deconstruct.unwrap();
            Ok(TransformLine {
                id,
                amount,
                location: ResLocationResolved {
                    id: loc_id,
                    amount: loc_amount,
                    radius,
                    lat,
                    lon,
                    unit_id,
                    unit: "",
                    res_name,
                }
            })
        }
    }
}

#[derive(Serialize)]
struct TransformLineContext<'a> {
    //types: &'a Vec<TransformType>,
    line: Option<TransformLine<'a>>,
    locations: Vec<ResLocationResolved<'a>>,
}

#[get("/transforms")]
pub fn transforms(conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Transform {
        id: u64,
        type_id: String,
        refer: String,
        lines: u64,
    }
    impl FromRow for Transform {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                Err(deconstruct.unwrap_err())
            } else {
                let (id, type_id, refer, lines) = deconstruct.unwrap();
                Ok(Transform {
                    id,
                    type_id,
                    refer,
                    lines
                })
            }
        }
    }

    let query_result = conn.prep_exec(fs::read_to_string("sql/transforms.sql").expect("file error"), ());

    let vec: Result<Vec<Transform>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) =>Template::render("transforms", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/addtransform")]
pub fn addtransform_page() -> Template {
    Template::render("transform", TransformContext { types: get_transform_types(), transform: None, locations: vec![] })
}

#[get("/addtransform?<refer>&<type_id>")]
pub fn addtransform(refer: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO transform_hdr (ref, type_id) VALUES (?, ?)", (refer, type_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Transformation added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/transform/<id>")]
pub fn transform(id: u64, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT id, type_id, ref FROM transform_hdr WHERE id = ?", (id,));
    let vec: Result<Vec<Transform>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut transform = vec.unwrap().remove(0);
    transform.type_name = &get_transform_types()[match_id(transform.type_id)].type_name;

    query_result = conn.prep_exec("SELECT transform_line.id, val, 0, 0.0, location.lat, location.lon, resource_location.loc_radius, qty_id, resource.name FROM transform_line \
    JOIN resource_location ON transform_line.res_loc_id = resource_location.id \
    JOIN location ON resource_location.loc_id = location.id \
    JOIN resource_param ON resource_location.res_param_id = resource_param.id \
    JOIN resource ON resource_param.res_id = resource.id \
    JOIN param ON resource_param.param_id = param.id \
    WHERE transform_hdr_id = ?", (id,));
    let vec: Result<Vec<TransformLine>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    transform.lines = vec.unwrap();
    for line in transform.lines.iter_mut() {
        line.location.unit = if line.location.unit_id == 0 { "" }
        else { &get_quantities()[match_id(line.location.unit_id)].unit }
    }

    query_result = conn.prep_exec("SELECT resource_location.id, loc_val, loc_radius, location.lat, location.lon, qty_id, resource.name FROM resource_location \
    JOIN resource_param ON resource_location.res_param_id = resource_param.id \
    JOIN resource ON resource_param.res_id = resource.id \
    JOIN param ON resource_param.param_id = param.id \
    JOIN location ON location.id = loc_id", ());
    let vec: Result<Vec<ResLocationResolved>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut locations = vec.unwrap();
    for location in locations.iter_mut() {
        location.unit = if location.unit_id == 0 { "" }
        else { &get_quantities()[match_id(location.unit_id)].unit }
    }

    Template::render("transform", TransformContext {
        types: &get_transform_types(),
        transform: Some(transform),
        locations,
    })
}

#[get("/modifytransform?<id>&<refer>&<type_id>")]
pub fn modifytransform(id: u64, refer: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE transform_hdr SET ref = ?, type_id = ? WHERE id = ?", (refer, type_id, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Transformation header modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/addline?<transform_id>&<amount>&<location>")]
pub fn addline(transform_id: u64, amount: f64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match insert_new_event(transform_id, amount, location, &conn) {
        Ok(m) => Flash::success(Redirect::to("/"), m),
        Err(e) => Flash::error(Redirect::to("/"), e),
    }
}

/*#[get("/line/<id>")]
pub fn line(id: u64, config: State<Config>, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT id, val, 0, resource_location.loc_val, location.lat, location.lon, resource_location.loc_radius, qty_id, resource.name FROM transform_line \
    JOIN resource_location ON transform_line.res_loc_id = resource_location.id \
    JOIN location ON resource_location.loc_id = location.id \
    JOIN resource ON resource_location.res_id = resource.id WHERE id = ?", (id,));
    let vec: Result<Vec<TransformLine>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut line = vec.unwrap().remove(0);
    line.location.unit = if line.location.unit_id == 0 { "" }
    else { &config.quantities[match_id(line.location.unit_id)].unit };

    query_result = conn.prep_exec("SELECT id, loc_val, loc_radius, location.lat, location.lon, res_param_id, resource.name FROM resource_location \
    JOIN resource ON resource.id = resource_location.res_id \
    JOIN location ON location.id = loc_id", ());
    let vec: Result<Vec<ResLocation>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }

    Template::render("line", TransformLineContext {
        line: Some(line),
        locations: vec.unwrap(),
    })
}*/

/*#[get("/modifyline?<id>&<amount>&<location>")]
pub fn modifyline(id: u64, amount: f64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE transform_line SET res_loc_id = ?, val = ? WHERE id = ?",
                                      (location, amount, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Transform line modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}*/

#[get("/deleteline/<id>")]
pub fn deleteline(id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    // get location and amount
    let (amount, location): (f64, u64) = match conn.first_exec("SELECT val, res_loc_id FROM transform_line WHERE id = ?", (id,)) {
        Ok(Some(row)) => (row.get(0).unwrap(), row.get(1).unwrap()),
        Ok(None) => return Flash::error(Redirect::to("/"), "No such transformation line."),
        Err(e) => return Flash::error(Redirect::to("/"), e.to_string()),
    };
    // update amount at location
    let mut query_result = conn.prep_exec("UPDATE resource_location SET loc_val = loc_val - ? WHERE id = ?", (amount, location));
    if query_result.is_err() {
        return Flash::error(Redirect::to("/"), query_result.unwrap_err().to_string());
    }

    query_result = conn.prep_exec("DELETE FROM transform_line WHERE id = ?", (id,));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Transform line removed."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/placeorder?<res_id>&<amount>&<location>")]
pub fn place_order(res_id: u64, amount:f64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    if res_is_available(res_id, amount, &conn) {
        res_move(res_id, amount,location, &conn);
        return Flash::success(Redirect::to("/"), "Resource delivered.");
    }
    match res_manufacture(res_id, amount, location, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource manufactured and delivered."),
        Err(e) => Flash::error(Redirect::to("/"), e),
    }
}

pub fn insert_new_event(transform_id: u64, amount: f64, location: u64, conn: &my::Pool) -> Result<&str, Cow<'static, str>> {
    if amount == 0.0 {
        return Err(Cow::Borrowed("Event cannot have 0 amount."))
    }
    // get original resource amount at location
    let orig_value: f64 = get_res_amount_at_location(location, &conn)?;
    // test for negative amount of resource at location
    if orig_value + amount < 0.0 {
        return Err(Cow::Borrowed("Amount at location must not be negative."))
    }
    // update amount at location
    update_res_amount_at_location(amount, location, &conn)?;
    // insert new transform event
    insert_new_event_db(transform_id, location, amount, &conn)
}

fn insert_new_event_db(transform_id: u64, location: u64, amount: f64, conn: &my::Pool) -> Result<&str, Cow<'static, str>> {
    match conn.prep_exec("INSERT INTO transform_line (transform_hdr_id, res_loc_id, val) VALUES (?, ?, ?)",
                         (transform_id, location, amount)) {
        Ok(_) => Ok("Transform event added."),
        Err(e) => Err(Cow::Owned(e.to_string())),
    }
}

fn res_is_available(res_id: u64, amount: f64, conn: &my::Pool) -> bool {
    unimplemented!()
}

fn res_move(res_id: u64, amount: f64, destination: u64, conn: &my::Pool) {
    unimplemented!()
}

fn res_manufacture(res_id: u64, amount:f64, destination: u64, conn: &my::Pool) -> Result<&str, &str> {
    struct ResAmount(u64, f64);

    fn res_get_dependencies(res_id: u64, conn: &my::Pool) -> Vec<ResAmount> {
        unimplemented!()
    }

    let deps = res_get_dependencies(res_id, conn);
    if deps.len() == 0 { return Err("Not enough resources.") }
    for dep in deps.iter() {
        if res_is_available(dep.0, dep.1, conn) {
            res_move(dep.0, dep.1, destination, conn);
        }
        else {
            if let Err(e) = res_manufacture(dep.0, dep.1, destination, conn) {
                return Err(e)
            }
        }
    }
    Ok("Resource manufactured.")
}

