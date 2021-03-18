use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::users;
use crate::schema::users::dsl::users as all_users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub first_name: String,
}

#[derive(Deserialize)]
pub struct UserData {
    pub username: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "users"]
pub struct UserNew {
    pub username: String,
    pub password: String,
    pub first_name: String,
}

impl User {
    pub fn get_all_users(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        all_users.order(users::id.desc()).load::<User>(conn)
    }

    pub fn insert_user(user: &UserNew, conn: &PgConnection) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(user)
            .get_result(conn)
    }

    pub fn get_user_by_username(
        user: UserData,
        conn: &PgConnection,
    ) -> Result<Vec<User>, diesel::result::Error> {
        all_users
            .filter(users::username.eq(user.username))
            .load::<User>(conn)
    }
}
