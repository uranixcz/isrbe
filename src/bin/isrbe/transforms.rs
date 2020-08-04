use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use isrbe::{catch_mysql_err, match_id, ERROR_PAGE, TransformType, get_transform_types, get_quantities};
use isrbe::locations::{ResLocationResolved, get_all_resource_locations, get_res_amount_at_location};
use std::borrow::Cow;
use isrbe::transforms::{TransformResolved, TransformLine, Transform, get_transforms, add_transform, get_transform, get_transform_lines, modify_transform, add_line, get_available_resource_locations, res_move, res_manufacture, get_line, delete_line};

#[derive(Serialize)]
struct TransformContext<'a> {
    types: &'a Vec<TransformType>,
    transform: Option<TransformResolved<'a>>,
    locations: Vec<ResLocationResolved<'a>>,
}

#[derive(Serialize)]
struct TransformLineContext<'a> {
    //types: &'a Vec<TransformType>,
    line: Option<TransformLine<'a>>,
    locations: Vec<ResLocationResolved<'a>>,
}

#[get("/transforms")]
pub fn transforms(conn: State<my::Pool>) -> Template {
    match get_transforms(&conn) {
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
    match add_transform(&refer, type_id, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Transformation added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/transform/<id>")]
pub fn transform(id: u64, conn: State<my::Pool>) -> Template {
    let transform = match get_transform(id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(t) => t,
    };

    let locations = match get_all_resource_locations(&conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(v) => v,
    };

    Template::render("transform", TransformContext {
        types: &get_transform_types(),
        transform: Some(transform),
        locations,
    })
}

#[get("/modifytransform?<id>&<refer>&<type_id>")]
pub fn modifytransform(id: u64, refer: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match modify_transform(id, refer, type_id, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Transformation header modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/addline?<transform_id>&<amount>&<location>")]
pub fn addline(transform_id: u64, amount: f64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    if amount == 0.0 {
        return Flash::error(Redirect::to("/"), Cow::Borrowed("Event cannot have 0 amount."));
    }
    // get original resource amount at location
    let orig_value: f64 = match get_res_amount_at_location(location, &conn) {
        Err(e) => return Flash::error(Redirect::to("/"), Cow::Owned(e.to_string())),
        Ok(None) => return Flash::error(Redirect::to("/"), Cow::Borrowed("No such resource location.")),
        Ok(Some(row)) => row.get(0).unwrap(),
    };
    // test for negative amount of resource at location
    if orig_value + amount < 0.0 {
        return Flash::error(Redirect::to("/"), Cow::Borrowed("Amount at location must not be negative."));
    }

    match add_line(transform_id, amount, location, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), Cow::Borrowed("Transform event added.")),
        Err(e) => Flash::error(Redirect::to("/"), Cow::Owned(e.to_string())),
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
    let (amount, location): (f64, u64) = match get_line(id, &conn) {
        Ok(Some(row)) => (row.get(0).unwrap(), row.get(1).unwrap()),
        Ok(None) => return Flash::error(Redirect::to("/"), "No such transformation line."),
        Err(e) => return Flash::error(Redirect::to("/"), e.to_string()),
    };

    match delete_line(id, location, amount, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Transform line removed."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/placeorder?<res_id>&<amount>&<location>")]
pub fn place_order(res_id: u64, amount:f64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match get_available_resource_locations(res_id, amount, &conn) {
        Err(e) => Flash::error(Redirect::to("/"), e),
        Ok(vec) => {
            if vec.is_empty() {
                match res_manufacture(res_id, amount, location, &conn) {
                    Ok(_) => Flash::success(Redirect::to("/"), "Resource manufactured and delivered."),
                    Err(e) => Flash::error(Redirect::to("/"), e),
                };
            }
            res_move(res_id, amount,location, &conn);
            Flash::success(Redirect::to("/"), "Resource delivered.")
        }
    }

}

