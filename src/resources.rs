use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, match_id, ERROR_PAGE, ResourceType, get_res_types};
use crate::locations::Coordinates;
use crate::parameters::Parameter;

#[derive(Serialize)]
struct ResourceContext<'a> {
    types: &'a Vec<ResourceType>,
    parameters: Vec<Parameter>,
    resource: Option<Resource<'a>>,
    coordinates: Vec<Coordinates>,
    parameter_list: Vec<(u64, String, String)>,
}

#[derive(Serialize, Debug)]
struct Resource <'a>{
    id: u64,
    name: String,
    type_id: u64,
    type_name: &'a str,
    //locations: Vec<ResLocation<'a>>,
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
                //locations: Vec::new(),
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
        parameters: u64,
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
                let (id, name, type_id, locations, parameters) = deconstruct.unwrap();
                Ok(Resource {
                    id,
                    name,
                    type_id,
                    locations,
                    parameters
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
pub fn addresource_page() -> Template {
    Template::render("resource", ResourceContext { types: get_res_types(), parameters: Vec::new(), resource: None, coordinates: Vec::new(), parameter_list: vec![] })
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
pub fn resource(id: u64, conn: State<my::Pool>) -> Template {
    let mut query_result = conn.prep_exec("SELECT id, name, type_id FROM resource WHERE id = ?", (id,));
    let vec: Result<Vec<Resource>, String> = catch_mysql_err(query_result);
    if vec.is_err() {
        return Template::render(ERROR_PAGE, vec.unwrap_err().to_string())
    }
    let mut resource = vec.unwrap().remove(0);
    resource.type_name = &get_res_types()[match_id(resource.type_id)].type_name;

    // get list of assigned parameters for location form
    query_result= conn.prep_exec(fs::read_to_string("sql/addreslocation.sql").expect("file error"), (id,));
    let params: Result<Vec<Parameter>, String> = catch_mysql_err(query_result);
    if params.is_err() {
        return Template::render(ERROR_PAGE, params.unwrap_err().to_string())
    }

    // get list of locations for location form
    query_result = conn.prep_exec("SELECT id, lat, lon FROM location", ());
    let coords: Result<Vec<Coordinates>, String> = catch_mysql_err(query_result);
    if coords.is_err() {
        return Template::render(ERROR_PAGE, coords.unwrap_err().to_string())
    }

    // get list of all parameters for assignment form
    // TODO move param_type to RAM on startup; Possible optimization to cache this list for addreslocation.sql above.
    query_result = conn.prep_exec("SELECT param.id, param.name, param_type.name FROM param JOIN param_type ON param.type = param_type.id", ());
    let parlist: Result<Vec<(u64, String, String)>, String> = catch_mysql_err(query_result);
    if parlist.is_err() {
        return Template::render(ERROR_PAGE, parlist.unwrap_err().to_string())
    }

    Template::render("resource", ResourceContext {
        types: &get_res_types(),
        parameters: params.unwrap(),
        resource: Some(resource),
        coordinates: coords.unwrap(),
        parameter_list: parlist.unwrap(),
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