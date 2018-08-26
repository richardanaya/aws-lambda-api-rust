resource "null_resource" "rust-api_zip" {
	triggers { uuid = "${uuid()}" } 
	provisioner "local-exec" {
		# Create zip if zip doesn't exist or binary is newer than zip
		command = "test -e ../lambdas/rust-api.zip && test -z `find ../lambdas/ -name rust-api -newer ../lambdas/rust-api.zip` || zip -9 -j ../lambdas/rust-api.zip ../lambdas/rust-api"
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

resource "aws_lambda_function" "rust-api_lambda" {
  filename         = "../lambdas/rust-api.zip"
  function_name    = "rust-api"
  role             = "${aws_iam_role.iam_for_lambda.arn}"
  handler          = "rust-api.main"
  source_code_hash = "${base64sha256(file("../lambdas/rust-api.zip"))}"
  runtime          = "go1.x"
  depends_on = ["null_resource.rust-api_zip"]
}