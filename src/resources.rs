use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, ERROR_PAGE, ResourceTypes, ResourceType};

#[derive(Serialize)]
struct ResourceContext<'a> {
    types: &'a ResourceTypes,
    resource: Option<Resource>,
}

#[derive(Serialize, Debug)]
struct Resource {
    id: u64,
    name: String,
    type_id: u64,
    type_name: String,
    locations: Vec<Location>,
}
#[derive(Serialize, Debug)]
struct Location {
    id: u64,
    value: f64,
    radius: u64,
    lat: f64,
    lon: f64,
    unit: String,
}

#[get("/resources")]
pub fn resources(conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Resource {
        id: u64,
        name: String,
        type_id: String,
        locations: u64,
        quantity: Option<f64>,
    }
    impl FromRow for Resource {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }

        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                return Err(deconstruct.unwrap_err());
            } else {
                let (id, name, type_id, locations, quantity) = deconstruct.unwrap();
                Ok(Resource {
                    id,
                    name,
                    type_id,
                    locations,
                    quantity
                })
            }
        }
    }

    let query_result = conn.prep_exec(fs::read_to_string("sql/resources.sql").expect("file error"), ());

    let vec: Result<Vec<Resource>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) => Template::render("resources", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}

#[get("/addresource")]
pub fn addresource_page<'a>(resource_types: State<ResourceTypes>) -> Template {


    Template::render("resource", ResourceContext { types: resource_types.inner(), resource: None })
}

#[get("/addresource?<name>&<type_id>")]
pub fn addresource(name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource (res_name, res_type_id) VALUES (?, ?)", (name, type_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/<id>")]
pub fn resource(id: u64, resource_types: State<ResourceTypes>, conn: State<my::Pool>) -> Template {
    impl FromRow for Resource {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                return Err(deconstruct.unwrap_err());
            } else {
                let (id, name, type_id) = deconstruct.unwrap();
                Ok(Resource {
                    id,
                    name,
                    type_id,
                    type_name: "".to_string(),
                    locations: Vec::new(),
                })
            }
        }
    }
    impl FromRow for Location {
        fn from_row(_row: my::Row) -> Self {
            unimplemented!()
        }
        fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                return Err(deconstruct.unwrap_err());
            } else {
                let (id, value, radius, lat, lon, unit) = deconstruct.unwrap();
                Ok(Location {
                    id,
                    value,
                    radius,
                    lat,
                    lon,
                    unit,
                })
            }
        }
    }
    let mut query_result = conn.prep_exec("SELECT res_id, res_name, res_type_id FROM resource WHERE res_id = ?", (id,));
    let vec: Result<Vec<Resource>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut resource_types = resource_types.clone();
    let mut resource = vec.unwrap().remove(0);
    let key = (resource.type_id - 1) as usize;
    resource.type_name = resource_types[key].type_name.to_string();
    resource_types.remove(key);
    resource_types.sort_unstable_by(|a, b| a.type_name.cmp(b.type_name));

    query_result = conn.prep_exec("SELECT res_loc_id, loc_val, loc_radius, loc_lat, loc_lon, quantity.qty_unit FROM resource_location \
    LEFT JOIN quantity ON quantity.qty_id = resource_location.res_qty_id WHERE res_id = ?", (id,));
    let vec: Result<Vec<Location>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    resource.locations = vec.unwrap();

    Template::render("resource", ResourceContext {
        types: &resource_types,
        resource: Some(resource)
    })
}

#[get("/modifyresource?<id>&<name>&<type_id>")]
pub fn modifyresource(id: u64, name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE resource SET res_name = ?, res_type_id =? WHERE res_id = ?", (name, type_id, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}