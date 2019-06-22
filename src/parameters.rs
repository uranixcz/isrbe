use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, Config, Quantity};

#[derive(Serialize, Debug)]
pub struct Parameter {
    id: u64,
    name: String,
    unit: String,
}
impl FromRow for Parameter {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, name, unit) = deconstruct.unwrap();
            Ok(Parameter {
                id,
                name,
                unit
            })
        }
    }
}

#[get("/parameters")]
pub fn parameters(config: State<Config>, conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Parameter<'a> {
        id: u64,
        name: String,
        type_id: String,
        unit_id: u64,
        unit: &'a str,
    }
    impl<'a> FromRow for Parameter<'a> {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                Err(deconstruct.unwrap_err())
            } else {
                let (id, name, type_id, unit_id) = deconstruct.unwrap();
                Ok(Parameter {
                    id,
                    name,
                    type_id,
                    unit_id,
                    unit: ""
                })
            }
        }
    }

    let query_result = conn.prep_exec(fs::read_to_string("sql/params.sql").expect("file error"), ());

    let vec: Result<Vec<Parameter>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(mut v) => {
            for p in v.iter_mut() {
                p.unit = if p.unit_id == 0 { "" }
                else { &config.quantities[match_id(p.unit_id)].unit }
            }
            Template::render("parameters", v)
        },
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/addparameter")]
pub fn addparameter_page(config: State<Config>) -> Template {
    Template::render("parameter", &config.quantities)
}

#[get("/addparameter?<name>&<type_id>&<unit>")]
pub fn addparameter(name: String, type_id: u64, unit: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO param (name, type, qty_id) VALUES (?, ?, ?)", (name, type_id, unit));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/addresparameter?<resource_id>&<param_id>&<movable>")]
pub fn addresparameter(resource_id: u64, param_id: u64, movable: bool, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource_param (res_id, param_id, is_movable) VALUES (?, ?, ?)",
                                      (resource_id, param_id, movable));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}



#[get("/resource/<id>/parameters")]
pub fn resparameters(id: u64, conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Parameter {
        id: u64,
        name: String,
        value: f64,
        unit: String,
        movable: bool,
    }
    impl FromRow for Parameter {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                Err(deconstruct.unwrap_err())
            } else {
                let (id, name, value, unit, movable) = deconstruct.unwrap();
                Ok(Parameter {
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

    let vec: Result<Vec<Parameter>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) => Template::render("resparams", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}