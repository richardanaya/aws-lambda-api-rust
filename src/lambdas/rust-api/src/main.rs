extern crate aws_lambda as lambda;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use failure::Error;
use lambda::event::apigw::ApiGatewayProxyRequest;
use lambda::Context;
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};
use std::sync::Mutex;

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

lazy_static! {
    static ref CONNECTION: Mutex<PgConnection> =  {
        let connection = establish_connection().unwrap();
        Mutex::new(connection)
    };
}

fn handle_request(
    e: ApiGatewayProxyRequest,
    _ctx: Context
) -> Result<serde_json::Value, Error> {
    let connection :&PgConnection = &CONNECTION.lock().unwrap();
    // query db
    let movies = diesel::sql_query("select * from movies").load::<Movie>(connection)?;

    // return a json structure api gateway expects for a 200 response
    Ok(json!({
      "statusCode":200,
      "body": format!(
          "List of movies for url path {}: {}",
          e.path, // The path of the current url (e.g. /index.html )
          movies  // Join the list of movie names with a ', '
              .iter()
              .map(|m| m.title.to_string())
              .collect::<Vec<_>>()
              .join(", ")
      )
    }))
}

pub fn establish_connection() -> Result<PgConnection, Error> {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient
        .get_secret_value(secret_request)
        .sync()?;
    let connection_string = match response.secret_string {
        Some(s) => s,
        None => return Err(format_err!("DB connection string is empty.")),
    };
    Ok(PgConnection::establish(&connection_string)?)
}



fn main() -> Result<(), Error> {
    lambda::start(move |e: ApiGatewayProxyRequest| {
        let ctx = Context::current();
        handle_request(e, ctx)
    });
    Ok(())
}
