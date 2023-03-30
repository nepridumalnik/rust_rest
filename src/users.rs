use crate::app_state::AppState;

use actix_web::{get, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde_json::{Map, Value};

static SELECT_BY_ID: &str = r"SELECT ID, Name, SecondName,
    Age, Male, Interests, City, Password, Email FROM Users WHERE ID = :id";

pub fn setup_services(config: &mut web::ServiceConfig) {
    config.service(get_by_id);
}

#[get("/user/get/{id}")]
async fn get_by_id(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id: u32 = path.into_inner();
    match data.users.pool.get_conn() {
        Ok(mut conn) => {
            let stmt = conn.prep(SELECT_BY_ID).unwrap();
            let param = params! {"id" => id};

            let mut row: Row = conn.exec_first(stmt, param).unwrap().unwrap();

            let mut person = Map::new();
            person.insert(
                "ID".to_string(),
                Value::from(row.take::<u32, _>("ID").unwrap()),
            );
            person.insert(
                "Name".to_string(),
                Value::from(row.take::<String, _>("Name").unwrap()),
            );
            person.insert(
                "SecondName".to_string(),
                Value::from(row.take::<String, _>("SecondName").unwrap()),
            );
            person.insert(
                "Age".to_string(),
                Value::from(row.take::<u32, _>("Age").unwrap()),
            );
            person.insert(
                "Male".to_string(),
                Value::from(row.take::<bool, _>("Male").unwrap()),
            );
            person.insert(
                "Interests".to_string(),
                Value::from(row.take::<String, _>("Interests").unwrap()),
            );
            person.insert(
                "City".to_string(),
                Value::from(row.take::<String, _>("City").unwrap()),
            );
            person.insert(
                "Password".to_string(),
                Value::from(row.take::<String, _>("Password").unwrap()),
            );
            person.insert(
                "Email".to_string(),
                Value::from(row.take::<String, _>("Email").unwrap()),
            );

            return HttpResponse::Ok().body(serde_json::to_string(&person).unwrap());
        }
        Err(error) => {
            return HttpResponse::NotFound().body(error.to_string());
        }
    }
}
