use lazy_static::lazy_static;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

lazy_static! {
    // the mutex make sure that the data is not accessed by multiple threads at the same time, so it is safe to use
    // and the data is mutable but it is global
    static ref ITEM_ARR: Mutex<Vec<Item>> = Mutex::new(Vec::new());
}
//Getting a form (object) from the query string (GET method)

#[derive(Clone, Deserialize, Serialize)]
pub struct Item {
    name: String,
    price: i8,
}

#[post("/items", format = "json", data = "<item>")]
pub fn item_func(item: Json<Item>) -> Json<Item> {
    let mut item_arr = ITEM_ARR.lock().unwrap();
    item_arr.push(item.into_inner());
    return Json(item_arr[item_arr.len() - 1].clone());
}

#[get("/items")]
pub fn get_items() -> Json<Vec<Item>> {
    let item_arr = ITEM_ARR.lock().unwrap();
    return Json(item_arr.clone());
}
