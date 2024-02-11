use rocket::http::RawStr;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
pub fn print_name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}
//that needs a wave argument and a query string of NAME, that can be Option<String> or String
#[get("/?wave&<name>")]
pub fn hello(name: Option<String>) -> String {
    //name.map maps, if there is a name, it will return the string with the name, if not, it will return the string "Hello!"
    name.map(|name| format!("Hi, {}!", name))
        .unwrap_or_else(|| "Hello!".into())
}
