use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types};

#[derive(Serialize, Debug)]
pub struct Parameter {
    id: u64,
    name: String,
    unit: String,
}
impl FromRow for Parameter {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, name, unit) = deconstruct.unwrap();
            Ok(Parameter {
                id,
                name,
                unit
            })
        }
    }
}

pub fn get_parameters(conn: &Pool) -> Result<Vec<(u64, String, String)>, String> {
    let query_result = conn.prep_exec("SELECT param.id, param.name, param_type.name FROM param JOIN param_type ON param.type = param_type.id", ());
    catch_mysql_err(query_result)
}