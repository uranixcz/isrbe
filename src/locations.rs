use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
//use std::fs;
use crate::{catch_mysql_err, ERROR_PAGE, Config, Quantity};

#[derive(Serialize)]
struct LocationContext<'a> {
    quantities: &'a Vec<Quantity>,
    location: Option<Location<'a>>,
}

#[derive(Serialize, Debug)]
pub struct Location<'a> {
    id: u64,
    amount: f64,
    radius: u64,
    lat: f64,
    lon: f64,
    pub unit_id: u64,
    pub unit: &'a str,
}
impl<'a> FromRow for Location<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            return Err(deconstruct.unwrap_err());
        } else {
            let (id, amount, radius, lat, lon, unit_id) = deconstruct.unwrap();
            Ok(Location {
                id,
                amount,
                radius,
                lat,
                lon,
                unit_id,
                unit: ""
            })
        }
    }
}

#[get("/addlocation?<resource_id>&<amount>&<unit>&<radius>&<lat>&<lon>")]
pub fn addlocation(resource_id: u64, amount: f64, unit: u64, radius: u64, lat: f64, lon: f64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource_location (res_id, res_qty_id, loc_lat, loc_lon, loc_radius, loc_val) VALUES (?, ?, ?, ?, ?, ?)",
                                      (resource_id, unit, lat, lon, radius, amount));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Location added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/location/<id>")]
pub fn location(id: u64, config: State<Config>, conn: State<my::Pool>) -> Template {
    let query_result = conn.prep_exec("SELECT res_loc_id, loc_val, loc_radius, loc_lat, loc_lon, res_qty_id FROM resource_location WHERE res_loc_id = ?", (id,));
    let vec: Result<Vec<Location>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut location = vec.unwrap().remove(0);
    location.unit = &config.quantities[location.unit_id as usize - 1].unit;
    println!("{} {}", location.unit, location.unit_id);

    Template::render("location", LocationContext {
        quantities: &config.quantities,
        location: Some(location)
    })
}