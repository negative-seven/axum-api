# axum_api

Simple, extendable REST API service utilizing the [axum](https://crates.io/crates/axum) framework.

## Usage

### Incorporated into custom server

Nest the Router returned by `create_api_router` to utilize this API handler as part of a larger service. An example of this can be seen in `main.rs`.

### As a standalone server

To run a simple server which merely maps all the API endpoints under `/api`, you must do the following:

- Set up a ScyllaDB instance. An appropriate empty database can be initialized with the script file located at `resources/init_database.cql`.
- Create a file containing a secret, which will be used to encode and decode JSON web tokens.
- Set up a configuration file. A complete configuration file with default values can be created with `cargo run -- -g`.

Finally, run `cargo run` to start the server.
