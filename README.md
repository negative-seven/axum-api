# axum_api

Simple, extendable API server utilizing the [axum](https://crates.io/crates/axum) framework.

## Prerequisites

The server requires a ScyllaDB database to be listening on `localhost:9042`. An appropriate empty database can be initialized with the script file located at `scripts/init_database.cql`.

## Usage

Run the server with the following command:

    cargo run
