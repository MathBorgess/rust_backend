# rust_backend

explain dependencies:

```toml
rocket -> The API framework
rocket_contrib -> For the JSON dealing with Rocket
serde -> For the JSON serialization
serde_json -> For the JSON serialization
diesel -> For the database connection
lazy_static -> For the global state
```

### Explain the use of the Rust programming language in the project

It's import to change the rust to Nightly, because the Rocket framework uses some features that are only available in the Nightly version of the Rust. To change the version of the rust, use the command:

```bash
rustup override set nightly
```

it is setting the rust version to nightly, but only local, to reset use:

```bash
rustup override unset
```

in the main.rs the ideal is to import all the controllers and the database connection, and then start the server with the Rocket framework.
Also, for the rust use, we need to import the libs in the main, so it can be used in the project, using the statements:

```rust
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
external crate <LIB_NAME>;

#[path = "<file_path>"]
mod MODULE_NAME;
```

And make sure to declare the functions, structs or any other thing that is going to be used in the project as pub (public).

### Explain the use of the Rocket framework in the project

The Rocket framework is used to create the API, and it is used to create the routes, and the controllers, and also the database connection. It is a very powerful framework, and it is very easy to use, and it is blazing fast.

In the file Rocket.toml, its possible to configure some infra settings, like the port, the address, the environment, and the log level.
[Rocket.toml website](https://rocket.rs/v0.4/guide/configuration/)

### Next steps!

The thing is to config the DIESEL ORM, and after upload this project to some cloud provider, and make the database connection with some database.
