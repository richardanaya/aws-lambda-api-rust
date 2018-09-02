extern crate aws_lambda as lambda;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate failure;
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use failure::Error;
use lambda::gateway::{Request, Response};
use lambda::Context;

mod db;

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

/// Handle API Gateway request and return API Gateway response.
fn handle_request(req: Request, _ctx: Context) -> Result<Response, Error> {
    let connection = &db::CONNECTION.lock().unwrap() as &PgConnection;
    let movies = diesel::sql_query("select * from movies").load::<Movie>(connection)?;

    Ok(lambda::gateway::response().status(200).body(
        format!(
            "List of movies for url path {}: {}",
            req.uri().path(), // The path of the current url (e.g. /index.html )
            movies  // Join the list of movie names with a ', '
                .iter()
                .map(|m| m.title.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ).into(),
    )?)
}

/// Start listening for AWS Lambda requests for API Gateway.
fn main() {
    lambda::gateway::start(|req| {
        let ctx = Context::current();
        handle_request(req, ctx)
    })
}
