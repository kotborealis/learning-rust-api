use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use super::models::users::{User, NewUser, UserData};
use serde_json::Value;

#[get("/", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
    let users = User::get_all_users(&conn);
    Json(json!({
        "status": 200,
        "result": users
    }))
}

#[get("/find?<username>")]
pub fn find_user(conn: DbConn, username: Option<String>) -> Json<Value> {
    match username {
        Some(username) => {
            println!("Find {}", username);
            let user_data = UserData { username };
            let users = User::get_user_by_username(user_data, &conn);
            Json(json!({
                "status": 200,
                "result": users
            }))
        },
        None => {
            Json(json!({
                "status": 400,
                "result": {}
            }))
        }
    }
}

#[post("/", format = "application/json", data = "<new_user>")]
pub fn new_user(conn: DbConn, new_user: Json<NewUser>) -> Json<Value> {
    let new_user = new_user.into_inner();
    let status = User::insert_user(&new_user, &conn);

    Json(json!({
        "status": if status { 200 } else { 400 },
        "result": new_user
    }))
}