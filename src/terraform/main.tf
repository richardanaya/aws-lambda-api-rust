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

resource "aws_lambda_function" "rust-api_lambda" {
  filename         = "../lambdas/rust-api.zip"
  function_name    = "rust-api"
  role             = "${aws_iam_role.iam_for_lambda.arn}"
  handler          = "rust-api"
  source_code_hash = "${base64sha256(file("../lambdas/rust-api.zip"))}"
  runtime          = "go1.x"
}
