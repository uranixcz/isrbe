use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use isrbe::{catch_mysql_err, match_id, ERROR_PAGE, get_quantities};
use isrbe::parameters::Parameter;
use isrbe::locations::{Coordinates, add_resource_location, get_resource_location_info, ResLocationResolved, get_locations, add_location, get_resource_locations, set_resource_amount_at_location};
use isrbe::locations::transport::*;
use mysql::Pool;

#[derive(Serialize)]
struct LocationContext<'a> {
    parameters: Vec<Parameter>,
    location: Option<ResLocationResolved<'a>>,
    coordinates: Vec<Coordinates>,
}

#[get("/addreslocation?<amount>&<res_param>&<radius>&<location>")]
pub fn addreslocation(amount: f64, res_param: u64, radius: u64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_resource_location(amount, res_param, radius, location, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource location added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/reslocation/<id>")]
pub fn reslocation(id: u64, conn: State<my::Pool>) -> Template {
    let location = match get_resource_location_info(id, &conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(l) => l,
    };

    /*query_result= conn.prep_exec(fs::read_to_string("sql/reslocation_list.sql").expect("file error"), (id,));
    let params: Result<Vec<Parameter>, String> = catch_mysql_err(query_result);
    if params.is_err() {
        return Template::render(ERROR_PAGE, params.unwrap_err().to_string())
    }*/
    let vec = match get_locations(&conn) {
        Err(e) => return Template::render(ERROR_PAGE, e),
        Ok(v) => v,
    };

    Template::render("reslocation", LocationContext {
        parameters: Vec::new(),
        location: Some(location),
        coordinates: vec,
    })
}

/*#[get("/modifyreslocation?<id>&<amount>&<res_param>&<radius>&<location>")]
pub fn modifyreslocation(id: u64, amount: f64, res_param: u64, radius: u64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE resource_location SET res_param_id = ?, loc_id = ?, loc_radius = ?, loc_val = ? WHERE id = ?",
                                      (res_param, location, radius, amount, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource location modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}*/
#[get("/modifyreslocation?<id>&<amount>")]
pub fn modifyreslocation(id: u64, amount: f64, conn: State<my::Pool>) -> Flash<Redirect> {
    match set_resource_amount_at_location(id, amount, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource location modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

/*#[get("/deletereslocation/<id>")]
pub fn deletereslocation(id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("DELETE FROM resource_location WHERE id = ?", (id,));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Location removed."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}*/

#[get("/addlocation")]
pub fn addlocation_page() -> Template {
    Template::render("location", ())
}

#[get("/addlocation?<lat>&<lon>")]
pub fn addlocation(lat: f64, lon: f64, conn: State<my::Pool>) -> Flash<Redirect> {
    match add_location(lat, lon, &conn) {
        Ok(_) => Flash::success(Redirect::to("/"), "Location added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/locations")]
pub fn locations(conn: State<my::Pool>) -> Template {
    match get_locations(&conn) {
        Ok(v) => Template::render("locations", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/resource/<id>/locations")]
pub fn reslocations(id: u64, conn: State<my::Pool>) -> Template {
    match get_resource_locations(id, &conn) {
        Ok(v) => Template::render("reslocations", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}