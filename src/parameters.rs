use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types, get_quantities};

pub const PARAM_TYPE_RESOURCE:u64 = 3;

#[derive(Serialize, Debug)]
pub enum Value {
    Number(f64),
    Text(String),
    Resource(f64, String),
    Empty
}

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
        let (id, name, unit) = my::from_row_opt(row)?;
        Ok(Parameter {
            id,
            name,
            unit
        })
    }
}

#[derive(Serialize, Debug)]
pub struct Parameter2<'a> {
    id: u64,
    name: String,
    type_id: String,
    pub unit_id: u64,
    pub unit: &'a str,
}
impl FromRow for Parameter2<'_> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, name, type_id, unit_id) = my::from_row_opt(row)?;
        Ok(Parameter2 {
            id,
            name,
            type_id,
            unit_id,
            unit: ""
        })
    }
}
impl Parameter2<'_> {
    fn set_unit_from_cache(&mut self) {
        if self.unit_id == 0 {
            self.unit = "";
        }
        else {
            self.unit = &get_quantities()[match_id(self.unit_id)].unit;
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Parameter3 {
    id: u64,
    name: String,
    value: Value,
    unit: Option<String>,
    movable: bool,
    res_param_id: u64,
}
impl FromRow for Parameter3 {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, name, val_f64, val_text, val_res, unit, movable, res_param_id) = my::from_row_opt(row)?;
        let value = match (val_f64, val_text, val_res) {
            (Some(x), None, None) => Value::Number(x),
            (None, Some(x), None) => Value::Text(x),
            (Some(x), None, Some(y)) => Value::Resource(x, y),
            _ => Value::Empty
        };
        Ok(Parameter3 {
            id,
            name,
            value,
            unit,
            movable,
            res_param_id,
        })
    }
}

pub fn get_parameters(conn: &Pool) -> Result<Vec<Parameter2>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/params.sql").expect("file error"), ());
    let mut parameters: Vec<Parameter2> = catch_mysql_err(query_result)?;
    for parameter in parameters.iter_mut() {
        parameter.set_unit_from_cache();
    }
    Ok(parameters)
}

pub fn get_resource_parameters(id: u64, conn: &Pool) -> Result<Vec<Parameter3>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/resparams.sql").expect("file error"), (id,));
    catch_mysql_err(query_result)
}

pub fn get_parameter_type(id: u64, conn: &Pool) -> Result<Vec<u64>, String> {
    let query_result = conn.prep_exec("SELECT type FROM param WHERE id = ? LIMIT 1", (id,));
    catch_mysql_err(query_result)
}

pub fn get_available_dependencies(res_param_id: u64, conn: &Pool) -> Result<Vec<Parameter>, String> {
    let query_result = conn.prep_exec("SELECT resource_param.id, resource.name, param.name FROM resource_param \
    JOIN resource ON resource.id = res_id JOIN param ON param.id = param_id \
    WHERE is_movable = 1 AND res_id != (SELECT res_id FROM resource_param WHERE id = ?)", (res_param_id,));
    catch_mysql_err(query_result)
}

pub fn get_parameter_type_by_res_param(res_param_id: u64, conn: &Pool) -> Result<Vec<u64>, String> {
    let query_result = conn.prep_exec("SELECT type FROM resource_param JOIN param ON param.id = resource_param.param_id WHERE resource_param.id = ?", (res_param_id,));
    catch_mysql_err(query_result)
}

/// Returns vector of Resource ID and amount
pub fn get_res_dependencies(res_id: u64, conn: &Pool) -> Result<Vec<(u64, f64)>, String> {
    let query_result = conn.prep_exec("SELECT rp2.res_id, val_float FROM resource_param \
        JOIN param_val ON param_val.res_param_id = resource_param.id \
        JOIN resource_param rp2 ON rp2.id = param_val.val_res \
        WHERE resource_param.res_id = ?", (res_id,));
    catch_mysql_err(query_result)
}

pub fn add_parameter(name: String, type_id: u64, mut unit: u64, conn: &Pool) -> my::Result<QueryResult> {
    if type_id != 1 {
        unit = 0;
    }
    conn.prep_exec("INSERT INTO param (name, type, qty_id) VALUES (?, ?, ?)", (name, type_id, unit))
}

pub fn add_res_parameter(resource_id: u64, param_id: u64, movable: bool, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO resource_param (res_id, param_id, is_movable) VALUES (?, ?, ?)", (resource_id, param_id, movable))
}

pub fn add_res_parameter_value_number(res_param_id: u64, value: f64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO param_val (res_param_id, val_float) VALUES (?, ?)", (res_param_id, value))
}

pub fn add_res_parameter_value_text(res_param_id: u64, value: String, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO param_val (res_param_id, val_text) VALUES (?, ?)", (res_param_id, value))
}

pub fn add_res_parameter_value_resource(res_param_id: u64, value: f64, dependency: u64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("INSERT INTO param_val (res_param_id, val_float, val_res) VALUES (?, ?, ?)", (res_param_id, value, dependency))
}