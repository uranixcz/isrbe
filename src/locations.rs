use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types, get_quantities};
use std::fs::OpenOptions;
use mysql::Row;

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
        let (id, lat, lon) = my::from_row_opt(row)?;
        Ok(Coordinates {
            id,
            lat,
            lon,
        })
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
impl FromRow for ResLocationResolved<'_> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, amount, radius, lat, lon, unit_id, res_name) = my::from_row_opt(row)?;
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
impl ResLocationResolved<'_> {
    fn set_unit_from_cache(&mut self) {
        if self.unit_id == 0 {
            self.unit = "";
        }
        else {
            self.unit = &get_quantities()[match_id(self.unit_id)].unit;
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ResLocationBasic {
    pub(crate) id: u64,
    pub amount: f64,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    radius: u64,
    unit: String
}
impl FromRow for ResLocationBasic {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, amount, radius, lat, lon, unit) = my::from_row_opt(row)?;
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
    let mut location: ResLocationResolved = catch_mysql_err(query_result)?.remove(0);
    location.set_unit_from_cache();
    Ok(location)
}

pub fn get_res_amount_at_location(location: u64, conn: &my::Pool) -> my::Result<Option<Row>> {
    conn.first_exec("SELECT loc_val FROM resource_location WHERE id = ?", (location,))
}

pub fn add_location(lat: f64, lon: f64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO location (lat, lon) VALUES (?, ?)", (lat, lon))
}

pub fn get_resource_locations(id: u64, conn: &Pool) -> Result<Vec<ResLocationBasic>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/reslocations.sql").expect("file error"), (id,));
    catch_mysql_err(query_result)
}

pub fn get_all_resource_locations(conn: &Pool) -> Result<Vec<ResLocationResolved>, String> {
    let query_result = conn.prep_exec("SELECT resource_location.id, loc_val, loc_radius, location.lat, location.lon, qty_id, resource.name FROM resource_location \
    JOIN resource_param ON resource_location.res_param_id = resource_param.id \
    JOIN resource ON resource_param.res_id = resource.id \
    JOIN param ON resource_param.param_id = param.id \
    JOIN location ON location.id = loc_id", ());
    let mut locations: Vec<ResLocationResolved>  = catch_mysql_err(query_result)?;
    for location in locations.iter_mut() {
        location.set_unit_from_cache();
    }
    Ok(locations)
}

pub fn set_resource_amount_at_location(id: u64, amount: f64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("UPDATE resource_location SET loc_val = ? WHERE id = ?", (amount, id))
}

pub fn distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    let distance_x = point1.0 - point2.0;
    let distance_y = point1.1 - point2.1;
    distance_x.hypot(distance_y)
}