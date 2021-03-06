use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types};
use crate::locations::Coordinates;
use crate::parameters::Parameter;

#[derive(Serialize, Debug)]
pub struct Resource {
    id: u64,
    name: String,
    pub type_id: u64,
    pub type_name: String,
    locations: u64,
    parameters: u64,
}
impl FromRow for Resource {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, name, type_id, type_name, locations, parameters) = my::from_row_opt(row)?;
        Ok(Resource {
            id,
            name,
            type_id,
            type_name,
            locations,
            parameters
        })
    }
}

pub fn get_resources(conn: &Pool) -> Result<Vec<Resource>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/resources.sql").expect("file error"), ());
    catch_mysql_err(query_result)
}

pub fn get_resource(id: u64, conn: &Pool) -> Result<Resource, String> {
    let query_result = conn.prep_exec("SELECT id, name, type_id, \"\", 999999, 999999 FROM resource WHERE id = ?", (id,));
    Ok(catch_mysql_err(query_result)?.remove(0))
}

pub fn get_assigned_parameters(id: u64, conn: &Pool) -> Result<Vec<Parameter>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/addreslocation.sql").expect("file error"), (id,));
    catch_mysql_err(query_result)
}

pub fn add_resource(name: String, type_id: u64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO resource (name, type_id) VALUES (?, ?)", (name, type_id))
}

pub fn modify_resource(id: u64, name: String, type_id: u64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("UPDATE resource SET name = ?, type_id = ? WHERE id = ?", (name, type_id, id))
}