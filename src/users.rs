use crate::app_state::AppState;

use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

static SELECT_BY_ID: &str = r"SELECT ID, Name, SecondName,
    Age, Male, Interests, City, Password, Email FROM Users WHERE ID = :id";
static INSERT_USER: &str = r"INSERT INTO Users(Name, SecondName, Age, Male, Interests, City, Password, Email)
    VALUES(?, ?, ?, ?, ?, ?, ?, ?)";
static SEARCH_USERS: &str = r"SELECT ID, Name, SecondName, Age, Male, Interests, City, Password, Email
    FROM Users WHERE Name LIKE :first_name AND SecondName LIKE :second_name";
static SELECT_USER_AUTH: &str = r"SELECT ID, Name, SecondName, Age, Male, Interests, City, Password, Email
    FROM Users WHERE Password = :password AND Email = :email";
static INSERT_AUTHORIZED: &str = r"INSERT INTO Tokens(ID, Token) VALUES(:id, :token)";

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

#[derive(Serialize, Deserialize)]
struct SelectedUser {
    id: u32,
    first_name: String,
    second_name: String,
    age: u32,
    male: bool,
    interests: String,
    city: String,
    password: String,
    email: String,
}

#[derive(Deserialize)]
struct SearchUser {
    first_name: String,
    second_name: String,
}

#[derive(Deserialize)]
struct LoginUser {
    password: String,
    email: String,
}

pub fn setup_services(config: &mut web::ServiceConfig) {
    config
        .service(get_by_id)
        .service(register_user)
        .service(search_user)
        .service(login);
}

#[post("/user/register")]
async fn register_user(data: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    match data.db.pool.get_conn() {
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

fn make_token(text: &String) -> String {
    let hasher = md5::compute(text.as_bytes());
    return format!("{:?}", hasher);
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

#[get("/user/get/{id}")]
async fn get_by_id(path: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id: u32 = path.into_inner();
    match data.db.pool.get_conn() {
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

#[get("/user/search")]
async fn search_user(data: web::Data<AppState>, search: web::Json<SearchUser>) -> impl Responder {
    match data.db.pool.get_conn() {
        Ok(mut conn) => {
            let stmt = conn.prep(SEARCH_USERS).unwrap();
            let param = params! {"first_name" => search.first_name.clone(), "second_name" => search.second_name.clone()};
            let result = conn.exec_iter(stmt, param).map(|result| {
                result.map(|x| x.unwrap()).map(|row| SelectedUser {
                    id: row.get(0).unwrap(),
                    first_name: row.get(1).unwrap(),
                    second_name: row.get(2).unwrap(),
                    age: row.get(3).unwrap(),
                    male: row.get(4).unwrap(),
                    interests: row.get(5).unwrap(),
                    city: row.get(6).unwrap(),
                    password: row.get(7).unwrap(),
                    email: row.get(8).unwrap(),
                })
            });

            let mut selected_users: Vec<SelectedUser> = Vec::new();
            let it = result.unwrap();

            for e in it {
                selected_users.push(e);
            }

            let json_string = serde_json::to_string(&selected_users).unwrap();

            return HttpResponse::Ok().body(json_string);
        }
        Err(error) => {
            return HttpResponse::BadRequest().body(error.to_string());
        }
    };
}

#[post("/login")]
async fn login(data: web::Data<AppState>, login: web::Json<LoginUser>) -> impl Responder {
    match data.db.pool.get_conn() {
        Ok(mut conn) => {
            let person: Map<String, Value>;

            {
                let stmt = conn.prep(SELECT_USER_AUTH).unwrap();
                let param =
                    params! {"password" => login.password.clone(), "email" => login.email.clone()};
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

                person = get_person(row).unwrap();
            }

            {
                let id = person["ID"].as_u64().unwrap() as u32;
                let token: String = make_token(&login.password);
                let stmt = conn.prep(INSERT_AUTHORIZED).unwrap();
                let param = params! {"id" => id, "token" => token.clone()};
                let result = conn.exec_drop(stmt, param);

                if result.is_err() {
                    return HttpResponse::Unauthorized().body("user was not authorized");
                }

                let json = json!(
                    {
                        "id": id,
                        "token": token
                    }
                );

                return HttpResponse::Ok().body(json.to_string());
            }
        }
        Err(error) => return HttpResponse::BadRequest().body(error.to_string()),
    }
}
