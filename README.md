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

### Explain the use of the Diesel ORM in the project

The Diesel ORM is used to connect the database with the project, and it is used to create the models, and the migrations, and also the database connection.
The issue here is that we need the lib to connect to the database (LIBPQ for Postgres) and also the diesel_cli to create the migrations and the models, so you will need to install everything to make it work even in the build of dockerfile.

You will have to call the diesel_cli to create the migrations and the schemas, and then you will have to call the diesel_cli to run the migrations. The command to create the migrations is:

```bash
diesel migration generate <MIGRATION_NAME>
```

After you complete of writing the up and down migration you could run, reset and redo:

```bash
    diesel migration run
    diesel migration reset
    diesel migration redo
```

### Next steps!

Make relation between the models, rearchitect the project as a good monolith api, make sure that the Rust container is working, and then deploy to the cloud.
