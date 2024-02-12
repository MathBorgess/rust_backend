#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate lazy_static;
extern crate rocket_contrib;
pub mod models;
pub mod schema;

#[path = "items/items_controller.rs"]
mod items_controller;
#[path = "root/root_controller.rs"]
mod root_controller;
#[path = "users/routes.rs"]
mod users;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                root_controller::index,
                root_controller::print_name,
                root_controller::hello,
                items_controller::item_func,
                items_controller::get_items,
                users::create::create_user,
                users::get_many::get_users,
                users::update::update_user,
                users::delete::delete_user,
            ],
        )
        .launch();
}
