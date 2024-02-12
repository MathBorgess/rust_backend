use crate::models::NewUser;
use api::establish_connection;
use diesel::RunQueryDsl;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct NewUserRequest<'a> {
    pub email: &'a str,
    pub name: &'a str,
    pub gender: bool,
    pub age: i32,
}

#[post("/users", format = "json", data = "<new_user>")]
pub fn create_user(new_user: Json<NewUserRequest>) -> Json<JsonValue> {
    let connection = &mut establish_connection();
    let new_user = NewUser {
        age: new_user.age,
        email: &new_user.email,
        name: &new_user.name,
        gender: new_user.gender,
        id: &Uuid::new_v4().to_string(),
    };

    diesel::insert_into(crate::schema::users::dsl::users)
        .values(&new_user)
        .execute(connection)
        .expect("Error saving new student");

    Json(JsonValue::from(serde_json::json!({
        "status": "success",
        "message": "Student has been created",
    })))
}
