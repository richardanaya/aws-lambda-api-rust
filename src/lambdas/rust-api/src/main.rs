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

#[derive(QueryableByName,Debug)]
#[table_name="movies"]
struct Movie {
    title: String
}

pub fn establish_connection() -> PgConnection {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient.get_secret_value(secret_request).sync().unwrap();
    let connection_string = response.secret_string.unwrap();
    PgConnection::establish(&connection_string).expect(&format!("Error connecting to postgresql"))
}

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()|{
        let connection = establish_connection();
        let results = diesel::sql_query("select * from movies").load::<Movie>(&connection);
        for rows in &results {
            for movie in rows {
                println!("{:?}",movie.title);
            }
        }
        Ok(json!({
          "statusCode":200,
          "body":"Connected!"
        }))
    })
}
