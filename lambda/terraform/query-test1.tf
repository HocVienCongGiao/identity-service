data "aws_s3_bucket_object" "exampleservice-test1-query-api" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
}


resource "aws_lambda_function" "exampleservice-test1-query-api" {
  s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test1.zip"
  function_name = "exampleservice-test1-query-api"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "test1"
  timeout       = 12

  vpc_config {
    # Every subnet should be able to reach an EFS mount target in the same Availability Zone. Cross-AZ mounts are not permitted.
    # subnet_ids         = [data.aws_subnet_ids.lambda.ids]
    subnet_ids         = ["subnet-02385ede395c1f51a", "subnet-0e6dd749246d5c65d", "subnet-00777fd51e0927323"]
    security_group_ids = ["sg-0f2acf7a4973e5d4c", " "]
  }

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = base64sha256(data.aws_s3_bucket_object.exampleservice-test1-query-api.last_modified)

  runtime = "provided"

  environment {
    variables = {
      API_KEY   = var.api_key
      TFE_TOKEN = var.tfe_token
    }
  }

  # Explicitly declare dependency on EFS mount target.
  # When creating or updating Lambda functions, mount target must be in 'available' lifecycle state.
  depends_on = [
    aws_iam_role_policy_attachment.lambda_logs,
    aws_cloudwatch_log_group.exampleservice-test1-query-api
  ]
}

resource "aws_cloudwatch_log_group" "exampleservice-test1-query-api" {
  name              = "/aws/lambda/exampleservice-test1-query-api" # Should be the same as function name
  retention_in_days = 14
}

