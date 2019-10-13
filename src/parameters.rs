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

#[derive(Serialize, Debug)]
enum Value {
    Number(f64),
    Text(String),
    Resource(u64),
    Empty
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
pub fn addparameter(name: String, type_id: u64, mut unit: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    if type_id != 1 {
        unit = 0;
    }
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
        value: Value,
        unit: Option<String>,
        movable: bool,
        res_param_id: u64,
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
                let (id, name, val_f64, val_text, val_res, unit, movable, res_param_id) = deconstruct.unwrap();
                let value = match (val_f64, val_text, val_res) {
                    (Some(x), None, None) => Value::Number(x),
                    (None, Some(x), None) => Value::Text(x),
                    (None, None, Some(x)) => Value::Resource(x),
                    _ => Value::Empty
                };
                Ok(Parameter {
                    id,
                    name,
                    value,
                    unit,
                    movable,
                    res_param_id,
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

#[get("/resource/parameter/<res_param_id>/addvalue", rank = 4)]
pub fn addresparametervalue_page(res_param_id: u64, conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct ParameterContext {
        resources: Vec<Parameter>,
        res_param_id: u64,
    }
    let query_result = conn.prep_exec("SELECT resource_param.id, resource.name, param.name FROM resource_param \
    JOIN resource ON resource.id = res_id JOIN param ON param.id = param_id", ());
    let vec: Result<Vec<Parameter>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) => Template::render("parameter_value", ParameterContext {
            resources: v,
            res_param_id,
        }),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}
//TODO delete other_id from template
#[get("/resource/parameter/<res_param_id>/addvalue?<value>&<other_id>")]
pub fn addresparametervaluenumber(res_param_id: u64, value: f64, other_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO param_val (res_param_id, val_float) VALUES (?, ?)", (res_param_id, value));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}
//TODO delete other_id from template
#[get("/resource/parameter/<res_param_id>/addvalue?<value>&<other_id>", rank = 2)]
pub fn addresparametervaluetext(res_param_id: u64, value: String, other_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO param_val (res_param_id, val_text) VALUES (?, ?)", (res_param_id, value));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/parameter/<res_param_id>/addvalue?<value>&<other_id>", rank = 3)]
pub fn addresparametervalueresource(res_param_id: u64, value: f64, other_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO param_val (res_param_id, val_float, val_res) VALUES (?, ?, ?)", (res_param_id, value, other_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}