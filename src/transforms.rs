use mysql as my;
use my::prelude::FromRow;
use my::{QueryResult, Pool};
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types, get_transform_types, get_quantities};
use crate::locations::{ResLocationResolved, get_res_amount_at_location, get_all_resource_locations, get_resource_locations, ResLocationBasic};
use std::borrow::Cow;
use crate::parameters::get_res_dependencies;

#[derive(Serialize, Debug)]
pub struct TransformResolved<'a> {
    id: u64,
    pub type_id: u64,
    pub type_name: &'a str,
    refer: String,
    pub lines: Vec<TransformLine<'a>>,
}
impl<'a> FromRow for TransformResolved<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, type_id, refer) = my::from_row_opt(row)?;
        Ok(TransformResolved {
            id,
            type_id,
            type_name: "",
            refer,
            lines: Vec::new(),
        })
    }
}

#[derive(Serialize, Debug)]
pub struct Transform {
    id: u64,
    type_id: String,
    refer: String,
    lines: u64,
}
impl FromRow for Transform {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, type_id, refer, lines) = my::from_row_opt(row)?;
        Ok(Transform {
            id,
            type_id,
            refer,
            lines
        })
    }
}

#[derive(Serialize, Debug)]
pub struct TransformLine<'a> {
    id: u64,
    amount: f64,
    pub location: ResLocationResolved<'a>,
}
impl<'a> FromRow for TransformLine<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let (id, amount, loc_id, loc_amount, lat, lon, radius, unit_id, res_name) = my::from_row_opt(row)?;
        Ok(TransformLine {
            id,
            amount,
            location: ResLocationResolved {
                id: loc_id,
                amount: loc_amount,
                radius,
                lat,
                lon,
                unit_id,
                unit: "",
                res_name,
            }
        })
    }
}

pub fn get_transform(id: u64, conn: &Pool) -> Result<TransformResolved, String> {
    let mut transform = get_transform_unresolved(id, &conn)?;
    transform.lines = get_transform_lines(id, &conn)?;
    for line in transform.lines.iter_mut() {
        line.location.unit = if line.location.unit_id == 0 { "" }
        else { &get_quantities()[match_id(line.location.unit_id)].unit }
    }
    Ok(transform)
}

fn get_transform_unresolved(id: u64, conn: &Pool) -> Result<TransformResolved, String> {
    let query_result = conn.prep_exec("SELECT id, type_id, ref FROM transform_hdr WHERE id = ?", (id,));
    let mut transform: TransformResolved = catch_mysql_err(query_result)?.remove(0);
    transform.type_name = &get_transform_types()[match_id(transform.type_id)].type_name;
    Ok(transform)
}

pub fn get_transforms(conn: &Pool) -> Result<Vec<Transform>, String> {
    let query_result = conn.prep_exec(fs::read_to_string("sql/transforms.sql").expect("file error"), ());
    catch_mysql_err(query_result)
}

pub fn get_transform_lines(id: u64, conn: &Pool) -> Result<Vec<TransformLine>, String> {
    let query_result = conn.prep_exec("SELECT transform_line.id, val, 0, 0.0, location.lat, location.lon, resource_location.loc_radius, qty_id, resource.name FROM transform_line \
    JOIN resource_location ON transform_line.res_loc_id = resource_location.id \
    JOIN location ON resource_location.loc_id = location.id \
    JOIN resource_param ON resource_location.res_param_id = resource_param.id \
    JOIN resource ON resource_param.res_id = resource.id \
    JOIN param ON resource_param.param_id = param.id \
    WHERE transform_hdr_id = ?", (id,));
    catch_mysql_err(query_result)
}

pub fn add_transform(refer: String, type_id: u64, conn: &Pool) -> Result<Vec<u64>, String> {
    let query_result = conn.prep_exec("INSERT INTO transform_hdr (ref, type_id) VALUES (?, ?); SELECT LAST_INSERT_ID();", (refer, type_id));
    catch_mysql_err(query_result)
}

pub fn modify_transform(id: u64, refer: String, type_id: u64, conn: &Pool) -> my::Result<QueryResult> {
    conn.prep_exec("UPDATE transform_hdr SET ref = ?, type_id = ? WHERE id = ?", (refer, type_id, id))
}

pub fn get_line(id: u64, conn: &Pool) -> Result<Option<my::Row>, my::Error> {
    conn.first_exec("SELECT val, res_loc_id FROM transform_line WHERE id = ?", (id,))
}

pub fn delete_line(id: u64, location: u64, amount: f64, conn: &Pool) -> my::Result<QueryResult> {
    // TODO should be in transaction
    let _ = conn.prep_exec("DELETE FROM transform_line WHERE id = ?", (id,))?;
    conn.prep_exec("UPDATE resource_location SET loc_val = loc_val - ? WHERE id = ?", (amount, location))
}

pub fn add_line(transform_id: u64, amount: f64, res_location: u64, conn: &Pool) -> Result<(), String> {
    // TODO should be in transaction
    if let Err(e) = conn.prep_exec("UPDATE resource_location SET loc_val = loc_val + ? WHERE id = ?", (amount, res_location)) {
        return Err(e.to_string())
    }
    if let Err(e) = conn.prep_exec("INSERT INTO transform_line (transform_hdr_id, res_loc_id, val) VALUES (?, ?, ?)", (transform_id, res_location, amount)) {
        return Err(e.to_string())
    } else {
        Ok(())
    }
}

pub fn get_available_resource_locations(res_id: u64, amount: f64, conn: &Pool) -> Result<Vec<ResLocationBasic>, String> {
    let locations = get_resource_locations(res_id, &conn)?;
    Ok(locations.iter().filter(|x| x.amount.ge(&amount)).cloned().collect::<Vec<ResLocationBasic>>())
}

pub fn res_move_auto(res_id: u64, amount: f64, destination: u64, transform_id: u64, conn: &Pool) -> Result<Option<u64>, String> { //TODO don't move if it's the same location
    let locations = get_available_resource_locations(res_id, amount, &conn)?;
    if locations.is_empty() {
        Ok(None)
    } else {
        let source = locations[0].id; // TODO insert routing algorithm here
        add_line(transform_id, -amount, source, &conn)?;
        add_line(transform_id, amount, destination, &conn)?;  // TODO transaction here
        Ok(Some(source))
    }
}

pub fn res_manufacture(res_id: u64, amount:f64, destination: u64, transform_id: u64, conn: &Pool) -> Result<(), String> {
    let deps = get_res_dependencies(res_id, conn)?;
    if deps.is_empty() { return Err("Not enough resources.".to_string()) }
    for dep in deps.iter() {
        match res_move_auto(dep.0, dep.1, destination, transform_id, conn) {
            Err(e) => return Err(e),
            Ok(None) => return res_manufacture(dep.0, dep.1, destination, transform_id, conn),
            Ok(Some(source)) => {
                add_line(transform_id, -dep.1, source, conn)?;
                add_line(transform_id, amount, destination, conn)?;
            }
        }

        /*let locations = get_available_resource_locations(dep.0, dep.1, conn)?;
        if locations.is_empty() {
            let _ = res_manufacture(dep.0, dep.1, destination, transform_id, conn)?;
        }
        else {
            res_move_auto(dep.0, dep.1, destination, conn)?;

        }*/
    }
    Ok(())
}