use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, ERROR_PAGE, Config, TransformType};

#[derive(Serialize)]
struct TransformContext<'a> {
    types: &'a Vec<TransformType>,
    transform: Option<Transform<'a>>,
}

#[derive(Serialize, Debug)]
struct Transform<'a> {
    id: u64,
    type_id: u64,
    type_name: &'a str,
    refer: String,
    lines: Vec<TransformLine>,
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
struct TransformLine {
    id: u64,
    amount: f64,
    lat: f64,
    lon: f64,
    radius: u64,
}
impl FromRow for TransformLine {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, amount, lat, lon, radius) = deconstruct.unwrap();
            Ok(TransformLine {
                id,
                amount,
                lat,
                lon,
                radius,
            })
        }
    }
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
pub fn addtransform_page(config: State<Config>) -> Template {
    Template::render("transform", TransformContext { types: &config.transform_types, transform: None})
}

#[get("/addtransform?<refer>&<type_id>")]
pub fn addtransform(refer: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO transform_hdr (transform_ref, transform_type_id) VALUES (?, ?)", (refer, type_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Transformation added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/transform/<id>")]
pub fn transform(id: u64, config: State<Config>, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT transform_hdr_id, transform_type_id, transform_ref FROM transform_hdr WHERE transform_hdr_id = ?", (id,));
    let vec: Result<Vec<Transform>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut transform = vec.unwrap().remove(0);
    transform.type_name = &config.transform_types[(transform.type_id - 1) as usize].type_name;

    query_result = conn.prep_exec("SELECT transform_line_id, transform_line_val, location.lat, location.lon, resource_location.loc_radius FROM transform_line \
    JOIN resource_location ON transform_line.res_loc_id = resource_location.res_loc_id \
    JOIN location ON resource_location.loc_id = location.id WHERE transform_hdr_id = ?", (id,));
    let vec: Result<Vec<TransformLine>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    transform.lines = vec.unwrap();

    Template::render("transform", TransformContext {
        types: &config.transform_types,
        transform: Some(transform),
    })
}