data "aws_caller_identity" "current" {}

terraform {
  backend "s3" {
    bucket = "richardanaya-remote-state"
    key    = "rust-api/terraform.tfstate"
    region = "us-east-1"
  }
}

module "rust-lambda" {
  source     = "../../.vendor/terraform/lambda"
  name       = "rust-api"
  zip_path  = "../lambdas/rust-api.zip"
  policy_statements = <<EOF
[
  {
      "Effect": "Allow",
      "Action": "secretsmanager:GetSecretValue",
      "Resource": "arn:aws:secretsmanager:us-east-1:${data.aws_caller_identity.current.account_id}:secret:elephantsql_connection-At46Hm"
  }
]
EOF
}

module "rust-api-gw" {
  source     = "../../.vendor/terraform/api-gw-single-lambda"
  name       = "rust-api"
  lambda-arn = "${module.rust-lambda.arn}"
}
