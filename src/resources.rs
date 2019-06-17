use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, Config, ResourceType, Quantity};
use crate::locations::{ResLocation, Coordinates};

#[derive(Serialize)]
struct ResourceContext<'a> {
    types: &'a Vec<ResourceType>,
    quantities: &'a Vec<Quantity>,
    resource: Option<Resource<'a>>,
    coordinates: Vec<Coordinates>,
}

#[derive(Serialize, Debug)]
struct Resource <'a>{
    id: u64,
    name: String,
    type_id: u64,
    type_name: &'a str,
    locations: Vec<ResLocation<'a>>,
}
impl<'a> FromRow for Resource<'a> {
    fn from_row(_row: my::Row) -> Self {
        unimplemented!()
    }
    fn from_row_opt(row: my::Row) -> Result<Self, my::FromRowError> {
        let deconstruct = my::from_row_opt(row);
        if deconstruct.is_err() {
            Err(deconstruct.unwrap_err())
        } else {
            let (id, name, type_id) = deconstruct.unwrap();
            Ok(Resource {
                id,
                name,
                type_id,
                type_name: "",
                locations: Vec::new(),
            })
        }
    }
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
                Err(deconstruct.unwrap_err())
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
pub fn addresource_page(config: State<Config>) -> Template {
    Template::render("resource", ResourceContext { types: &config.resource_types, quantities: &Vec::new(), resource: None, coordinates: Vec::new() })
}

#[get("/addresource?<name>&<type_id>")]
pub fn addresource(name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("INSERT INTO resource (name, type_id) VALUES (?, ?)", (name, type_id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource added."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}

#[get("/resource/<id>")]
pub fn resource(id: u64, config: State<Config>, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT id, name, type_id FROM resource WHERE id = ?", (id,));
    let vec: Result<Vec<Resource>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut resource = vec.unwrap().remove(0);
    resource.type_name = &config.resource_types[match_id(resource.type_id)].type_name;

    query_result = conn.prep_exec("SELECT id, loc_val, loc_radius, location.lat, location.lon, res_param_id, \"\" \
    FROM resource_location JOIN location ON loc_id = location.id WHERE res_id = ?", (id,));
    let vec: Result<Vec<ResLocation>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    resource.locations = vec.unwrap();
    for location in resource.locations.iter_mut() {
        location.unit = if location.unit_id == 0 { "" }
        else { &config.quantities[match_id(location.unit_id)].unit }
    }
    query_result = conn.prep_exec("SELECT id, lat, lon FROM location", ());
    let vec: Result<Vec<Coordinates>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }

    Template::render("resource", ResourceContext {
        types: &config.resource_types,
        quantities: &config.quantities,
        resource: Some(resource),
        coordinates: vec.unwrap(),
    })
}

#[get("/modifyresource?<id>&<name>&<type_id>")]
pub fn modifyresource(id: u64, name: String, type_id: u64, conn: State<my::Pool>) -> Flash<Redirect> {
    let query_result = conn.prep_exec("UPDATE resource SET name = ?, type_id = ? WHERE id = ?", (name, type_id, id));
    match query_result {
        Ok(_) => Flash::success(Redirect::to("/"), "Resource modified."),
        Err(e) => Flash::error(Redirect::to("/"), e.to_string())
    }
}