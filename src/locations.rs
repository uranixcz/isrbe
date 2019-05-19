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
    location: Option<ResLocation<'a>>,
    coordinates: Vec<Coordinates>,
}

#[derive(Serialize, Debug)]
pub struct ResLocation<'a> {
    pub id: u64,
    pub amount: f64,
    pub radius: u64,
    pub lat: f64,
    pub lon: f64,
    pub unit_id: u64,
    pub unit: &'a str,
    pub res_name: String,
}
impl<'a> FromRow for ResLocation<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, amount, radius, lat, lon, unit_id, res_name) = deconstruct.unwrap();
            Ok(ResLocation {
                id,
                amount,
                radius,
                lat,
                lon,
                unit_id,
                unit: "",
                res_name,
            })
        }
    }
}
#[derive(Serialize, Debug)]
pub struct Coordinates {
    id: u64,
    lat: f64,
    lon: f64,
}
impl FromRow for Coordinates {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, lat, lon) = deconstruct.unwrap();
            Ok(Coordinates {
                id,
                lat,
                lon,
            })
        }
    }
}

#[get("/addreslocation?<resource_id>&<amount>&<unit>&<radius>&<location>")]
pub fn addreslocation(resource_id: u64, amount: f64, unit: u64, radius: u64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource_location (res_id, res_qty_id, loc_id, loc_radius, loc_val) VALUES (?, ?, ?, ?, ?)",
                                      (resource_id, unit, location, radius, amount));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource location added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/reslocation/<id>")]
pub fn reslocation(id: u64, config: State<Config>, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT res_loc_id, loc_val, loc_radius, location.lat, location.lon, res_qty_id, \"\" \
    FROM resource_location JOIN location ON loc_id = location.id WHERE res_loc_id = ?", (id,));
    let vec: Result<Vec<ResLocation>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut location = vec.unwrap().remove(0);
    location.unit = if location.unit_id == 0 { "" }
    else { &config.quantities[location.unit_id as usize - 1].unit };

    query_result = conn.prep_exec("SELECT id, lat, lon FROM location", ());
    let vec: Result<Vec<Coordinates>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }

    Template::render("reslocation", LocationContext {
        quantities: &config.quantities,
        location: Some(location),
        coordinates: vec.unwrap(),
    })
}

#[get("/modifyreslocation?<id>&<amount>&<unit>&<radius>&<location>")]
pub fn modifyreslocation(id: u64, amount: f64, unit: u64, radius: u64, location: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE resource_location SET res_qty_id = ?, loc_id = ?, loc_radius = ?, loc_val = ? WHERE res_loc_id = ?",
                                      (unit, location, radius, amount, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource location modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

/*#[get("/deletereslocation/<id>")]
pub fn deletereslocation(id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("DELETE FROM resource_location WHERE res_loc_id = ?", (id,));
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
    let query_result = conn.prep_exec("INSERT INTO location (lat, lon) VALUES (?, ?)", (lat, lon));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Location added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/locations")]
pub fn locations(conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Location {
        id: u64,
        lat: f64,
        lon: f64,
    }
    impl FromRow for Location {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                Err(deconstruct.unwrap_err())
            } else {
                let (id, lat, lon) = deconstruct.unwrap();
                Ok(Location {
                    id,
                    lat,
                    lon,
                })
            }
        }
    }

    let query_result = conn.prep_exec("SELECT * FROM location", ());

    let vec: Result<Vec<Location>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) => Template::render("locations", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}