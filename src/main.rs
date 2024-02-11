#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate lazy_static;

#[path = "items/items_controller.rs"]
mod items_controller;
#[path = "root/root_controller.rs"]
mod root_controller;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                root_controller::index,
                root_controller::print_name,
                root_controller::hello,
                items_controller::item_func,
                items_controller::get_items
            ],
        )
        .launch();
}
