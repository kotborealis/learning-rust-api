use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::db::schema::users;
use crate::db::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

impl User {
    pub fn get_all(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        all_users.order(users::id.desc()).load::<User>(conn)
    }

    pub fn create(conn: &PgConnection, user: &NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
    }

    pub fn get_by_id(conn: &PgConnection, id: i32) -> Result<User, diesel::result::Error> {
        users::table.find(id).first::<User>(conn)
    }
}
