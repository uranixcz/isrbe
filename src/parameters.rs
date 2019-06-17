use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
//use std::fs;
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
