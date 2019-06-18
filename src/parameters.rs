use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, Config, Quantity};

/*#[get("/addparam")]
pub fn addparam_page(config: State<Config>) -> Template {
    Template::render("resource", ResourceContext { types: &config.resource_types, quantities: &Vec::new(), resource: None, coordinates: Vec::new() })
}*/

#[get("/addparam?<name>&<type_id>")]
pub fn addparam(name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO param (name, type) VALUES (?, ?)", (name, type_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/addresparam?<resource_id>&<param_id>&<movable>")]
pub fn addresparam(resource_id: u64, param_id: u64, movable: bool, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource_param (res_id, param_id, is_movable) VALUES (?, ?, ?)",
                                      (resource_id, param_id, movable));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/<id>/params")]
pub fn resparams(id: u64, conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Param {
        id: u64,
        name: String,
        value: f64,
        unit: String,
        movable: bool,
    }
    impl FromRow for Param {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                Err(deconstruct.unwrap_err())
            } else {
                let (id, name, value, unit, movable) = deconstruct.unwrap();
                Ok(Param {
                    id,
                    name,
                    value,
                    unit,
                    movable,
                })
            }
        }
    }

    let query_result = conn.prep_exec(fs::read_to_string("sql/resparams.sql").expect("file error"), (id,));

    let vec: Result<Vec<Param>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) => Template::render("resparams", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}