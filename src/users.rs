use crate::app_state::AppState;

use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::Deserialize;
use serde_json::{Map, Value};

static SELECT_BY_ID: &str = r"SELECT ID, Name, SecondName,
    Age, Male, Interests, City, Password, Email FROM Users WHERE ID = :id";
static INSERT_USER: &str = r"INSERT INTO Users(Name, SecondName, Age, Male, Interests, City, Password, Email)
    VALUES(?, ?, ?, ?, ?, ?, ?, ?)";

#[derive(Deserialize)]
struct User {
    first_name: String,
    second_name: String,
    age: u32,
    male: bool,
    interests: String,
    city: String,
    password: String,
    email: String,
}

pub fn setup_services(config: &mut web::ServiceConfig) {
    config.service(get_by_id).service(register_user);
}

fn get_person(row: Row) -> std::result::Result<Map<String, Value>, MySqlError> {
    let result = std::panic::catch_unwind(move || {
        let mut row: Row = row;
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

        return person;
    });

    match result {
        Ok(person) => return Ok(person),
        Err(_) => {
            return Err(MySqlError {
                code: 0,
                message: "Failed when getting user".to_string(),
                state: "Failed".to_string(),
            })
        }
    }
}

#[post("/user/register")]
async fn register_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    match data.users.pool.get_conn() {
        Ok(mut conn) => {
            let stmt = conn.prep(INSERT_USER).unwrap();
            let res = conn.exec_drop(
                stmt,
                (
                    user.first_name.clone(),
                    user.second_name.clone(),
                    user.age,
                    user.male,
                    user.interests.clone(),
                    user.city.clone(),
                    user.password.clone(),
                    user.email.clone(),
                ),
            );

            match res {
                Ok(()) => {
                    return HttpResponse::Ok().body("");
                }
                Err(error) => {
                    return HttpResponse::BadRequest().body(error.to_string());
                }
            }
        }
        Err(error) => {
            return HttpResponse::BadRequest().body(error.to_string());
        }
    };
}

#[get("/user/get/{id}")]
async fn get_by_id(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id: u32 = path.into_inner();
    match data.users.pool.get_conn() {
        Ok(mut conn) => {
            let stmt = conn.prep(SELECT_BY_ID).unwrap();
            let param = params! {"id" => id};
            let row: Row;

            match conn.exec_first(stmt, param) {
                Ok(option) => match option {
                    Some(user) => row = user,
                    None => return HttpResponse::NotFound().body("user not found"),
                },
                Err(_) => {
                    return HttpResponse::NotFound().body("user not found");
                }
            };

            match get_person(row) {
                Ok(person) => {
                    return HttpResponse::Ok().body(serde_json::to_string(&person).unwrap())
                }
                Err(error) => return HttpResponse::NotFound().body(error.to_string()),
            }
        }
        Err(error) => {
            return HttpResponse::NotFound().body(error.to_string());
        }
    }
}
