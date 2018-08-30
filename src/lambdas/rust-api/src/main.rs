extern crate aws_lambda as lambda;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate diesel;

use failure::Error;
use failure::ResultExt;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::*;
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};

#[derive(QueryableByName)]
struct Movie {
    #[sql_type = "Text"]
    title: String,
}

pub fn establish_connection() -> Result<PgConnection, Error> {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient
        .get_secret_value(secret_request)
        .sync()
        .context("Could not get db connection string.")?;
    let connection_string = match response.secret_string {
        Some(s) => s,
        None => return Err(format_err!("DB connection string is empty.")),
    };
    Ok(PgConnection::establish(&connection_string).context("Could not connect to database.")?)
}

fn main() -> Result<(),Error> {
    let connection = establish_connection()?;
    lambda::start(move |()| {
        let results = diesel::sql_query("select * from movies").load::<Movie>(&connection)?;
        Ok(json!({
          "statusCode":200,
          "body": format!("List of movies: {}", results.iter().map(|m| m.title.to_string()).collect::<Vec<_>>().join(", "))
        }))
    });
    Ok(())
}
