use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use std::fs;
use isrbe::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types};
use isrbe::resources::*;
use isrbe::locations::Coordinates;
use isrbe::parameters::{Parameter, get_parameters};
use isrbe::locations::get_locations;

#[derive(Serialize)]
struct ResourceContext<'a> {
    types: &'a Vec<ResourceType>,
    parameters: Vec<Parameter>,
    resource: Option<ResourceBasic<'a>>,
    coordinates: Vec<Coordinates>,
    parameter_list: Vec<(u64, String, String)>,
}

#[get("/resources")]
pub fn resources(conn: State<my::Pool>) -> Template {
    match get_resources(&conn) {
        Ok(v) => Template::render("resources", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/addresource")]
pub fn addresource_page() -> Template {
    Template::render("resource", ResourceContext { types: get_res_types(), parameters: Vec::new(), resource: None, coordinates: Vec::new(), parameter_list: vec![] })
}

#[get("/addresource?<name>&<type_id>")]
pub fn addresource(name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_resource(name, type_id, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/<id>")]
pub fn resource(id: u64, conn: State<my::Pool>) -> Template {
    let mut resource = match get_resource(id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(r) => r,
    };
    // TODO make this more universal in struct? Or automatic in get_resource?
    resource.type_name = &get_res_types()[match_id(resource.type_id)].type_name;

    // get list of assigned parameters for location form
    let params = match get_assigned_parameters(id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(v) => v,
    };

    // get list of locations for location form
    let coords = match get_locations(&conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(v) => v,
    };

    // get list of all parameters for assignment form
    // TODO move param_type to RAM on startup; Possible optimization to cache this list for addreslocation.sql above.
    let paramlist = match get_parameters(&conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(v) => v,
    };

    Template::render("resource", ResourceContext {
        types: &get_res_types(),
        parameters: params,
        resource: Some(resource),
        coordinates: coords,
        parameter_list: paramlist,
    })
}

#[get("/modifyresource?<id>&<name>&<type_id>")]
pub fn modifyresource(id: u64, name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match modify_resource(id, name, type_id, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}