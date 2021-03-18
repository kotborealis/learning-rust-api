use super::db::Conn as DbConn;
use super::models::users::{User, UserData, UserNew};
use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket_contrib::json::Json;

#[get("/")]
pub fn get_all(conn: DbConn) -> Result<status::Accepted<Json<Vec<User>>>, Status> {
    User::get_all_users(&conn)
        .map(|users| status::Accepted(Some(Json(users))))
        .map_err(|error| error_status(error))
}

#[get("/find?<username>")]
pub fn find_user(
    conn: DbConn,
    username: String,
) -> Result<status::Accepted<Json<Vec<User>>>, Status> {
    let user_data = UserData { username };
    User::get_user_by_username(user_data, &conn)
        .map(|users| status::Accepted(Some(Json(users))))
        .map_err(error_status)
}

#[post("/", data = "<new_user>")]
pub fn new_user(
    conn: DbConn,
    new_user: Json<UserNew>,
) -> Result<status::Created<Json<User>>, Status> {
    let new_user = new_user.into_inner();

    User::insert_user(&new_user, &conn)
        .map(|new_user| status::Created("Created user".to_string(), Some(Json(new_user))))
        .map_err(error_status)
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}
