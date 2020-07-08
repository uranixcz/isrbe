#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
extern crate mysql;

use serde::Serialize;
use std::fmt::Debug;
use mysql::prelude::FromRow;
use mysql as my;

pub mod locations;
pub mod resources;
pub mod transforms;
pub mod parameters;

pub const ERROR_PAGE: &str = "error";
pub static mut QUANTITIES: Vec<Quantity> = Vec::new();
pub static mut RESOURCE_TYPES: Vec<ResourceType> = Vec::new();
pub static mut TRANSFORM_TYPES: Vec<TransformType> = Vec::new();
pub fn get_res_types() -> &'static Vec<ResourceType> {
    unsafe { &RESOURCE_TYPES }
}
pub fn get_quantities() -> &'static Vec<Quantity> {
    unsafe { &QUANTITIES }
}
pub fn get_transform_types() -> &'static Vec<TransformType> {
    unsafe { &TRANSFORM_TYPES }
}

#[derive(Serialize, Debug)]
pub struct ResourceType {
    id: u64,
    type_name: String,
}
impl FromRow for ResourceType {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, type_name) = deconstruct.unwrap();
            Ok(ResourceType {
                id,
                type_name
            })
        }
    }
}

#[derive(Serialize, Debug)]
pub struct TransformType {
    id: u64,
    type_name: String,
}
impl FromRow for TransformType {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, type_name) = deconstruct.unwrap();
            Ok(TransformType {
                id,
                type_name
            })
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Quantity {
    id: u64,
    name: String,
    unit: String,
}
impl FromRow for Quantity {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, name, unit) = deconstruct.unwrap();
            Ok(Quantity {
                id,
                name,
                unit,
            })
        }
    }
}

pub fn catch_mysql_err<T: Serialize + Debug + FromRow>(query_result: Result<my::QueryResult, my::Error>) -> Result<Vec<T>, String> {
    if query_result.is_err() {
        return Err(query_result.unwrap_err().to_string());
    }
    let mut vec: Vec<T> = Vec::new();
    for result in query_result.unwrap() {
        match result {
            Err(e) => return Err(e.to_string()),
            Ok(row) => {
                let deconstruct = T::from_row_opt(row);
                if deconstruct.is_err() {
                    return Err(deconstruct.unwrap_err().to_string());
                }
                vec.push(deconstruct.unwrap());
            }
        }
    }
    Ok(vec)
}

pub fn match_id(id: u64) -> usize {
    id as usize - 1
}
