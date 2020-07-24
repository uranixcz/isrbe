use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types};

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

pub fn get_locations(conn: &Pool) -> Result<Vec<Coordinates>, String> {
    let  mut query_result = conn.prep_exec("SELECT id, lat, lon FROM location", ());
    catch_mysql_err(query_result)
}