use crate::{models::UpdateUser, schema::users::dsl::*};
use api::establish_connection;
use diesel::prelude::*;
use rocket::http::RawStr;
use rocket_contrib::json::{Json, JsonValue};

#[put("/users/<user_id>", data = "<update_data>")]
pub fn update_user(user_id: &RawStr, update_data: Json<UpdateUser>) -> Json<JsonValue> {
    let connection = &mut establish_connection();

    let _updated_users = diesel::update(users.find(user_id.as_str()))
        .set(&update_data.into_inner())
        .execute(connection)
        .expect("Failed to update student");

    // Return a JSON response indicating success
    Json(JsonValue::from(serde_json::json!({
        "users": "success",
        "message": format!("Student {} has been updated", user_id),
    })))
}
