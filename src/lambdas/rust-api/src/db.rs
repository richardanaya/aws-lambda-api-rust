use diesel::pg::PgConnection;
use diesel::prelude::*;
use failure::Error;
use rusoto_core::Region;
use rusoto_secretsmanager::{GetSecretValueRequest, SecretsManager, SecretsManagerClient};
use std::sync::Mutex;

/// Global database connection.
lazy_static! {
    pub static ref CONNECTION: Mutex<PgConnection> = {
        let connection = establish_connection().unwrap();
        Mutex::new(connection)
    };
}

/// Retrieve connection string from AWS Secrets and connect to PostgreSQL.
pub fn establish_connection() -> Result<PgConnection, Error> {
    let secretclient = SecretsManagerClient::new(Region::UsEast1);
    let mut secret_request = GetSecretValueRequest::default();
    secret_request.secret_id = "elephantsql_connection".to_owned();
    let response = secretclient.get_secret_value(secret_request).sync()?;
    let connection_string = match response.secret_string {
        Some(s) => s,
        None => return Err(format_err!("DB connection string is empty.")),
    };
    Ok(PgConnection::establish(&connection_string)?)
}
