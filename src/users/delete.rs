use crate::schema::users::dsl::*;
use api::establish_connection;
use diesel::prelude::*;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

#[delete("/users/<user_id>")]
pub fn delete_user(user_id: &RawStr) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    diesel::delete(users.find(id))
        .execute(connection)
        .expect(&format!("Unable to find student {}", user_id.as_str()));

    Json(JsonValue::from(serde_json::json!({
        "status": "success",
        "message": format!("Student with ID {} has been deleted", user_id.as_str()),
    })))
}
