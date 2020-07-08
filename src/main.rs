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
use isrbe::locations::*;
use isrbe::resources::*;
use isrbe::transforms::*;
use isrbe::parameters::*;
use isrbe::{catch_mysql_err, QUANTITIES, RESOURCE_TYPES, TRANSFORM_TYPES};

const ERROR_PAGE: &str = "error";

enum Language {
    English,
    Czech,
}

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

    let resource_count = conn.first_exec("SELECT COUNT(id) from resource",()).unwrap().unwrap().get(0).unwrap();
    let transform_count = conn.first_exec("SELECT COUNT(id) from transform_hdr",()).unwrap().unwrap().get(0).unwrap();

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
        .attach(AdHoc::on_attach("db_url", |rocket| {
            let db_url = rocket.config().get_str("db_url").expect("Please set db_url = \"mysql://...\" in Rocket.toml");
            let pool = my::Pool::new(db_url).unwrap();
            unsafe {
                QUANTITIES = catch_mysql_err(pool.prep_exec("SELECT id, name, unit FROM quantity", ())).unwrap();
                RESOURCE_TYPES = catch_mysql_err(pool.prep_exec("SELECT res_type_id, res_type_name FROM resource_type", ())).unwrap();
                TRANSFORM_TYPES = catch_mysql_err(pool.prep_exec("SELECT id, name FROM transform_type", ())).unwrap();
            }
            Ok(rocket.manage(pool))
        }))
        .mount("/", routes![index, resources, resource, addresource_page, addresource, modifyresource,
        addreslocation, reslocation, modifyreslocation, reslocations,
        locations, addlocation_page, addlocation,
        transforms, transform, addtransform_page, addtransform, modifytransform, addline, deleteline, place_order,
        parameters, addparameter, addresparameter, resparameters, addparameter_page, addresparametervalue_page, addresparametervaluenumber, addresparametervaluetext, addresparametervalueresource])
        .mount("/static", rocket_contrib::serve::StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}