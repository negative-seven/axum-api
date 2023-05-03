# axum_api

Simple, extendable REST API service utilizing the [axum](https://crates.io/crates/axum) framework.

## Usage

### Incorporated into custom server

Nest the Router returned by `create_api_router` to utilize this API handler as part of a larger service. An example of this can be seen in `main.rs`.

### As a standalone server

To run a simple server which merely maps all the API endpoints under `/api`, you must a ScyllaDB instance listening on `localhost:9042`. An appropriate empty database can be initialized with the script file located at `scripts/init_database.cql`. Then, run the following command:

    cargo run
