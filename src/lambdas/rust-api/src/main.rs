extern crate aws_lambda as lambda;
#[macro_use]
extern crate serde_json;
extern crate rusoto_core;
extern crate rusoto_secretsmanager;

use rusoto_core::Region;
use rusoto_secretsmanager::{SecretsManager, SecretsManagerClient,GetSecretValueRequest};

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()|{
        let secretclient = SecretsManagerClient::new(Region::UsEast1);
        let mut secret_request = GetSecretValueRequest::default();
        secret_request.secret_id = "elephantsql_connection".to_owned();
        let connection_string = secretclient.get_secret_value(secret_request).sync()?.secret_string.unwrap();
        Ok(
        json!({
          "statusCode":200,
          "body":connection_string
      })
    )
}
    )
}
