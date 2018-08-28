data "aws_caller_identity" "current" {}

terraform {
  backend "s3" {
    bucket = "richardanaya-remote-state"
    key    = "rust-api/terraform.tfstate"
    region = "us-east-1"
  }
}

resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_iam_role_policy" "test_policy" {
  name = "test_policy"
  role = "${aws_iam_role.iam_for_lambda.id}"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
        "Effect": "Allow",
        "Action": "secretsmanager:GetSecretValue",
        "Resource": "arn:aws:secretsmanager:us-east-1:${data.aws_caller_identity.current.account_id}:secret:elephantsql_connection-At46Hm"
    }
  ]
}
EOF
}

resource "aws_lambda_function" "rust-api_lambda" {
  filename         = "../lambdas/rust-api.zip"
  function_name    = "rust-api"
  role             = "${aws_iam_role.iam_for_lambda.arn}"
  handler          = "rust-api"
  source_code_hash = "${base64sha256(file("../lambdas/rust-api.zip"))}"
  runtime          = "go1.x"
}

resource "aws_api_gateway_rest_api" "api" {
  name = "rust-api"
}

# Root
resource "aws_api_gateway_method" "method_root" {
  rest_api_id   = "${aws_api_gateway_rest_api.api.id}"
  resource_id   = "${aws_api_gateway_rest_api.api.root_resource_id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "integration_root" {
  rest_api_id             = "${aws_api_gateway_rest_api.api.id}"
  resource_id             = "${aws_api_gateway_rest_api.api.root_resource_id}"
  http_method             = "${aws_api_gateway_method.method_root.http_method}"
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/${aws_lambda_function.rust-api_lambda.arn}/invocations"
}

# /{proxy+}
resource "aws_api_gateway_resource" "resource" {
  path_part = "{proxy+}"
  parent_id = "${aws_api_gateway_rest_api.api.root_resource_id}"
  rest_api_id = "${aws_api_gateway_rest_api.api.id}"
}

resource "aws_api_gateway_method" "method" {
  rest_api_id   = "${aws_api_gateway_rest_api.api.id}"
  resource_id   = "${aws_api_gateway_resource.resource.id}"
  http_method   = "ANY"
  authorization = "NONE"
}

resource "aws_api_gateway_integration" "integration" {
  rest_api_id             = "${aws_api_gateway_rest_api.api.id}"
  resource_id             = "${aws_api_gateway_resource.resource.id}"
  http_method             = "${aws_api_gateway_method.method.http_method}"
  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = "arn:aws:apigateway:us-east-1:lambda:path/2015-03-31/functions/${aws_lambda_function.rust-api_lambda.arn}/invocations"
}

resource "aws_lambda_permission" "apigw_lambda" {
  statement_id  = "AllowExecutionFromAPIGatewayRoot"
  action        = "lambda:InvokeFunction"
  function_name = "${aws_lambda_function.rust-api_lambda.arn}"
  principal     = "apigateway.amazonaws.com"
  source_arn = "arn:aws:execute-api:us-east-1:${data.aws_caller_identity.current.account_id}:${aws_api_gateway_rest_api.api.id}/*/*"
}

resource "aws_api_gateway_deployment" "MyDemoDeployment" {
  depends_on = ["aws_api_gateway_integration.integration","aws_api_gateway_integration.integration_root"]

  rest_api_id = "${aws_api_gateway_rest_api.api.id}"
  stage_name  = "api"
}
