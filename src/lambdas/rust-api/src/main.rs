extern crate aws_lambda as lambda;
#[macro_use]
extern crate serde_json;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use rusoto_core::Region;
use rusoto_secretsmanager::{SecretsManager, SecretsManagerClient,GetSecretValueRequest};

table! {
    movies (title) {
        title -> Varchar,
    }
}

#[derive(QueryableByName)]
#[table_name="movies"]
struct Movie {
    title: String
}

pub fn establish_connection() -> PgConnection {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let secret_request = GetSecretValueRequest {
        secret_id : "elephantsql_connection".to_owned(),
        ..Default::default()
    };
    let response = secretclient.get_secret_value(secret_request).sync().unwrap();
    let connection_string = response.secret_string.unwrap();
    PgConnection::establish(&connection_string).expect(&format!("Error connecting to postgresql"))
}

fn main() {
    let connection: PgConnection = establish_connection();

    lambda::start(move |()|{
        let results = diesel::sql_query("select * from movies").load::<Movie>(&connection);
        Ok(json!({
          "statusCode":200,
          "body": format!("List of movies: {}", results.unwrap().iter().map(|ref m| m.title.to_string()).collect::<Vec<_>>().join(", "))
        }))
    })
}
