use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, ERROR_PAGE, ResourceTypes};

#[get("/addlocation?<id>")]
pub fn addlocation(id: u64, conn: State<my::Pool>) -> Template {
    Template::render(ERROR_PAGE, ())
}