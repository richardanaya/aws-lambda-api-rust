extern crate aws_lambda as lambda;
#[macro_use]
extern crate serde_json;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;
extern crate postgres;
use postgres::{Connection, TlsMode};

use rusoto_core::Region;
use rusoto_secretsmanager::{SecretsManager, SecretsManagerClient,GetSecretValueRequest};

pub fn establish_connection() -> Connection {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient.get_secret_value(secret_request).sync().unwrap();
    let connection_string = response.secret_string.unwrap();
    Connection::connect(connection_string, TlsMode::None).unwrap()
}

struct Movie {
    title: String
}

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()|{
        let conn = establish_connection();
        for row in &conn.query("SELECT title FROM movies ", &[]).unwrap() {
            let movie = Movie {
                title: row.get(0)
            };
            println!("Found movie {}",movie.title);
        }
        Ok(json!({
          "statusCode":200,
          "body":"Connected!"
        }))
    })
}
