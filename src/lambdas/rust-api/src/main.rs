extern crate aws_lambda as lambda;
#[macro_use]
extern crate serde_json;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_types::*;
use rusoto_core::Region;
use rusoto_secretsmanager::{SecretsManager, SecretsManagerClient,GetSecretValueRequest};

#[derive(QueryableByName)]
struct Movie {
    #[sql_type="Text"]
    title: String
}

pub fn establish_connection() -> Result<PgConnection,ConnectionError> {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient.get_secret_value(secret_request).sync().unwrap();
    let connection_string = response.secret_string.unwrap();
    PgConnection::establish(&connection_string)
}

fn main() {
    let connection = establish_connection().expect("Could not connect to database.");
    lambda::start(move |()|{
        let results = diesel::sql_query("select * from movies").load::<Movie>(&connection)?;
        Ok(json!({
          "statusCode":200,
          "body": format!("List of movies: {}", results.iter().map(|m| m.title.to_string()).collect::<Vec<_>>().join(", "))
        }))
    })
}
