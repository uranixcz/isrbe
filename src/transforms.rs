use rocket_contrib::templates::Template;
use rocket::State;
use rocket::response::{Flash, Redirect};
use mysql as my;
use my::prelude::FromRow;
use std::fs;
use crate::{catch_mysql_err, ERROR_PAGE, ResourceTypes};

#[get("/transforms")]
pub fn transforms(conn: State<my::Pool>) -> Template {
    #[derive(Serialize, Debug)]
    struct Transform {
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
            let deconstruct = my::from_row_opt(row);
            if deconstruct.is_err() {
                return Err(deconstruct.unwrap_err());
            } else {
                let (id, type_id, refer, lines) = deconstruct.unwrap();
                Ok(Transform {
                    id,
                    type_id,
                    refer,
                    lines
                })
            }
        }
    }

    let query_result = conn.prep_exec(fs::read_to_string("sql/transforms.sql").expect("file error"), ());

    let vec: Result<Vec<Transform>, String> = catch_mysql_err(query_result);
    match vec {
        Ok(v) =>Template::render("transforms", v),
        Err(e) => Template::render(ERROR_PAGE, e)
    }
}