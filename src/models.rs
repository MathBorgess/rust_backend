use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub gender: bool,
    pub age: i32,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub email: &'a str,
    pub name: &'a str,
    pub gender: bool,
    pub age: i32,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateUser {
    email: Option<String>,
    name: Option<String>,
    gender: Option<bool>,
    age: Option<i32>,
}
