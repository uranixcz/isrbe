use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types, get_quantities};

pub mod transport;

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

#[derive(Serialize, Debug)]
pub struct ResLocationResolved<'a> {
    pub id: u64,
    pub amount: f64,
    pub radius: u64,
    pub lat: f64,
    pub lon: f64,
    pub unit_id: u64,
    pub unit: &'a str,
    pub res_name: String,
}
impl<'a> FromRow for ResLocationResolved<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, amount, radius, lat, lon, unit_id, res_name) = deconstruct.unwrap();
            Ok(ResLocationResolved {
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
pub struct ResLocationBasic {
    id: u64,
    amount: f64,
    lat: f64,
    lon: f64,
    radius: u64,
    unit: String
}
impl FromRow for ResLocationBasic {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, amount, radius, lat, lon, unit) = deconstruct.unwrap();
            Ok(ResLocationBasic {
                id,
                amount,
                lat,
                lon,
                radius,
                unit,
            })
        }
    }
}

pub fn get_locations(conn: &Pool) -> Result<Vec<Coordinates>, String> {
    let  query_result = conn.prep_exec("SELECT id, lat, lon FROM location", ());
    catch_mysql_err(query_result)
}

pub fn add_resource_location(amount: f64, res_param: u64, radius: u64, location: u64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO resource_location (res_param_id, loc_id, loc_radius, loc_val) VALUES (?, ?, ?, ?)",
                                      (res_param, location, radius, amount))
}

pub fn get_resource_location_info(id: u64, conn: &Pool) -> Result<ResLocationResolved, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/reslocation.sql").expect("file error"), (id,));
    Ok(catch_mysql_err(query_result)?.remove(0))
}

pub fn add_location(lat: f64, lon: f64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO location (lat, lon) VALUES (?, ?)", (lat, lon))
}

pub fn get_locations_of_resource(id: u64, conn: &Pool) -> Result<Vec<ResLocationBasic>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/reslocations.sql").expect("file error"), (id,));
    catch_mysql_err(query_result)
}

pub fn get_resource_locations(conn: &Pool) -> Result<Vec<ResLocationResolved>, String> {
    let query_result = conn.prep_exec("SELECT resource_location.id, loc_val, loc_radius, location.lat, location.lon, qty_id, resource.name FROM resource_location \
    JOIN resource_param ON resource_location.res_param_id = resource_param.id \
    JOIN resource ON resource_param.res_id = resource.id \
    JOIN param ON resource_param.param_id = param.id \
    JOIN location ON location.id = loc_id", ());
    let mut locations: Vec<ResLocationResolved>  = catch_mysql_err(query_result)?;
    for location in locations.iter_mut() {
        location.unit = if location.unit_id == 0 { "" }
        else { &get_quantities()[match_id(location.unit_id)].unit }
    }
    Ok(locations)
}