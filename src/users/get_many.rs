use crate::{models::User, schema::users::dsl::*};
use api::establish_connection;
use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

#[get("/users")]
pub fn get_users() -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let users_values = users
        .load::<User>(connection)
        .expect("Error loading students");

    Json(JsonValue::from(serde_json::json!(users_values)))
}
