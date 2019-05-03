/*
* Copyright 2019 Michal Mauser
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate mysql;

use rocket::{Rocket, State};
use rocket::fairing::AdHoc;
use rocket_contrib::templates::Template;
use rocket::request::FlashMessage;
use mysql as my;
use serde::Serialize;
use std::fmt::Debug;
use rocket::response::{Flash, Redirect};
use my::prelude::FromRow;
use locations::*;
use resources::*;
use transforms::*;

mod locations;
mod resources;
mod transforms;

const ERROR_PAGE: &'static str = "error";

enum Language {
    English,
    Czech,
}

#[derive(Serialize, Clone)]
pub struct ResourceType {
    id: u64,
    type_name: &'static str,
}

type ResourceTypes = Vec<ResourceType>;

#[get("/")]
fn index(flash: Option<FlashMessage>, conn: State<my::Pool>) -> Template {
    #[derive(Serialize)]
    struct Overview<'a> {
        resource_count: u64,
        transform_count: u64,
        message: Option<&'a str>,
    }
    impl<'a> Overview<'a> {
        fn message(mut self, msg: &'a str) -> Self { self.message = Some(msg); self}
    }

    let resource_count = conn.first_exec("SELECT COUNT(res_id) from resource",()).unwrap().unwrap().get(0).unwrap();
    let transform_count = conn.first_exec("SELECT COUNT(transform_hdr_id) from transform_hdr",()).unwrap().unwrap().get(0).unwrap();

    let overview = Overview {
        resource_count,
        transform_count,
        message: None
    };

     if let Some(x) = flash {
        Template::render("index", overview.message(x.msg()))
    } else { Template::render("index", overview) }
}

fn rocket() -> Rocket {
    let resource_types = vec![ResourceType {id: 1, type_name: "Natural"}, ResourceType {id: 2, type_name: "Transport"}, ResourceType {id: 3, type_name: "Energy"}, ResourceType {id: 4, type_name: "Production"}];
    rocket::ignite()
        .attach(Template::fairing())
        .attach(AdHoc::on_attach("template_dir",|rocket| {
            println!("Adding token managed state from config...");
            let language = match rocket.config().get_str("template_dir") {
                Ok("templates_cz") => Language::Czech,
                _ => Language::English,
            };
            Ok(rocket.manage(language))
        }))
        .attach(AdHoc::on_attach("db_url",|rocket| {
            let db_url = rocket.config().get_str("db_url").expect("Please set db_url = \"mysql://...\" in Rocket.toml");
            let pool = my::Pool::new(db_url).unwrap();
            Ok(rocket.manage(pool))
        }))
        .manage(resource_types)
        .mount("/", routes![index, resources, resource, addresource_page, addresource, modifyresource,
        addlocation,
        transforms])
        .mount("/static", rocket_contrib::serve::StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}

fn catch_mysql_err<T: Serialize + Debug + FromRow>(query_result: Result<my::QueryResult, my::Error>) -> Result<Vec<T>, String> {
    if query_result.is_err() {
        return Err(query_result.unwrap_err().to_string());
    }
    let mut vec: Vec<T> = Vec::new();
    for result in query_result.unwrap() {
        match result {
            Err(e) => return Err(e.to_string()),
            Ok(row) => {
                let deconstruct = T::from_row_opt(row);
                if deconstruct.is_err() {
                    return Err(deconstruct.unwrap_err().to_string());
                }
                vec.push(deconstruct.unwrap());
            }
        }
    }
    Ok(vec)
}