extern crate aws_lambda as lambda;
#[macro_use]
extern crate serde_json;

fn main() {
    // start the runtime, and return a greeting every time we are invoked
    lambda::start(|()| Ok(
        json!({
          "statusCode":200,
          "body":"Hello World!"
      })
    ))
}
