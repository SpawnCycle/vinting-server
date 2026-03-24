//! How to not get executed when making a contribution to this repo:
//!   - Think
//!   - Read the errors the compiler gives you
//!   - Read the warnings the compiler and linter give you
//!   - Avoid code repetition where possible (with traits or macros)
//!   - Structure things properly

pub mod config;
pub mod constants;
pub mod database;
pub mod dotenv;
pub mod file_server;
pub mod jwt;
pub mod responder;
pub mod routable_file_server;
pub mod routes;
