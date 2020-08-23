/*
* Copyright 2019-2020 Michal Mauser
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

#[macro_use] extern crate serde_derive;
extern crate mysql;

pub mod resources;
pub mod parameters;
pub mod locations;
pub mod transforms;

use serde::Serialize;
use std::fmt::Debug;
use mysql::prelude::FromRow;
use mysql as my;

pub const ERROR_PAGE: &str = "error";
pub static mut QUANTITIES: Vec<Quantity> = Vec::new();
pub static mut RESOURCE_TYPES: Vec<ResourceType> = Vec::new(); // probably legacy
pub static mut TRANSFORM_TYPES: Vec<TransformType> = Vec::new();
pub fn init(pool: &my::Pool) {
    unsafe {
        QUANTITIES = catch_mysql_err(pool.prep_exec("SELECT id, name, unit FROM quantity", ())).unwrap();
        RESOURCE_TYPES = catch_mysql_err(pool.prep_exec("SELECT res_type_id, res_type_name FROM resource_type", ())).unwrap();
        TRANSFORM_TYPES = catch_mysql_err(pool.prep_exec("SELECT id, name FROM transform_type", ())).unwrap();
    }
}
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
    pub type_name: String,
}
impl FromRow for ResourceType {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, type_name) = my::from_row_opt(row)?;
        Ok(ResourceType {
            id,
            type_name
        })
    }
}

#[derive(Serialize, Debug)]
pub struct TransformType {
    pub id: u64,
    pub type_name: String,
}
impl FromRow for TransformType {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, type_name) = my::from_row_opt(row)?;
        Ok(TransformType {
            id,
            type_name
        })
    }
}

#[derive(Serialize, Debug)]
pub struct Quantity {
    id: u64,
    name: String,
    pub unit: String,
}
impl FromRow for Quantity {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, name, unit) = my::from_row_opt(row)?;
        Ok(Quantity {
            id,
            name,
            unit,
        })
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
