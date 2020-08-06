use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use isrbe::{catch_mysql_err, match_id, ERROR_PAGE, get_quantities};
use isrbe::parameters::{Parameter, PARAM_TYPE_RESOURCE, Value, add_parameter, Parameter2, Parameter3, get_parameters, get_resource_parameters, add_res_parameter_value_number, add_res_parameter_value_text, add_res_parameter_value_resource, get_parameter_type, add_res_parameter, get_available_dependencies, get_parameter_type_by_res_param};

#[get("/parameters")]
pub fn parameters(conn: State<my::Pool>) -> Template {
    match get_parameters(&conn) {
        Ok(v) => Template::render("parameters", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/addparameter")]
pub fn addparameter_page() -> Template {
    Template::render("parameter", get_quantities())
}

#[get("/addparameter?<name>&<type_id>&<unit>")]
pub fn addparameter(name: String, type_id: u64, unit: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_parameter(name, type_id, unit, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/addresparameter?<resource_id>&<param_id>&<movable>")]
pub fn addresparameter(resource_id: u64, param_id: u64, movable: bool, conn: State<my::Pool>) -> Flash<Redirect> {
        match get_parameter_type(param_id, &conn) {
        Err(e) => return Flash::error(Redirect::to("/"), e.to_string()),
        Ok(t) => {
            if movable && t[0] != PARAM_TYPE_RESOURCE {
                return Flash::error(Redirect::to("/"), "Only numeric parameters are transportable.")
            }
        }
    }
    match add_res_parameter(resource_id, param_id, movable, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource parameter added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}



#[get("/resource/<id>/parameters")]
pub fn resparameters(id: u64, conn: State<my::Pool>) -> Template {
    match get_resource_parameters(id, &conn) {
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
        is_type_resource: bool,
    }
    let resources = match get_available_dependencies(res_param_id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e.to_string()),
        Ok(v) => v,
    };
    let param_type = match get_parameter_type_by_res_param(res_param_id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e.to_string()),
        Ok(v) => v,
    };
    Template::render("parameter_value", ParameterContext {
        resources,
        res_param_id,
        is_type_resource: param_type[0] == PARAM_TYPE_RESOURCE,
    })
}

#[get("/resource/parameter/<res_param_id>/addvalue?<value>", rank = 3)] //TODO no more than one value if transportable
pub fn addresparametervaluenumber(res_param_id: u64, value: f64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_res_parameter_value_number(res_param_id, value, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/parameter/<res_param_id>/addvalue?<value>", rank = 2)]
pub fn addresparametervaluetext(res_param_id: u64, value: String, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_res_parameter_value_text(res_param_id, value, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/parameter/<res_param_id>/addvalue?<value>&<dependency>", rank = 1)]
pub fn addresparametervalueresource(res_param_id: u64, value: f64, dependency: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_res_parameter_value_resource(res_param_id, value, dependency, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Parameter value added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}