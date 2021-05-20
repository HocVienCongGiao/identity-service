/*
The "REST API" is the container for all of the other API Gateway objects we will create.
All incoming requests to API Gateway must match with a configured resource and method in order to be handled. 
Append the following to define a single proxy resource:
*/
resource "aws_api_gateway_resource" "exampleservice-test2-query-api-proxy" {
  rest_api_id = data.aws_api_gateway_rest_api.query-api.id
  parent_id   = aws_api_gateway_resource.exampleservice-query-api-proxy.id
  path_part   = "test2"
}
resource "aws_api_gateway_method" "exampleservice-test2-query-api-proxy" {
  rest_api_id   = data.aws_api_gateway_rest_api.query-api.id
  resource_id   = aws_api_gateway_resource.exampleservice-test2-query-api-proxy.id
  http_method   = "ANY"
  authorization = "NONE"
}

  resource "aws_api_gateway_integration" "exampleservice-test2-query-api" {
  rest_api_id = data.aws_api_gateway_rest_api.query-api.id
  resource_id = aws_api_gateway_method.exampleservice-test2-query-api-proxy.resource_id
  http_method = aws_api_gateway_method.exampleservice-test2-query-api-proxy.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.exampleservice-test2-query-api.invoke_arn
}
  

  resource "aws_lambda_permission" "exampleservice-test2-query-api-gateway" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.exampleservice-test2-query-api.function_name
  principal     = "apigateway.amazonaws.com"

  # The "/*/*" portion grants access from any method on any resource
  # within the API Gateway REST API.
  source_arn = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
} 

data "aws_s3_bucket_object" "exampleservice-test2" {
  bucket = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  key    = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
}


resource "aws_lambda_function" "exampleservice-test2-query-api" {
  s3_bucket     = "${var.aws_account_id}-${var.aws_region}-aws-lambda"
  s3_key        = "dev-sg-hocvienconggiao/example-service/latest/test2.zip"
  function_name = "exampleservice-test2"
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "test2"
  timeout       = 12

  # The filebase64sha256() function is available in Terraform 0.11.12 and later
  # For Terraform 0.11.11 and earlier, use the base64sha256() function and the file() function:
  # source_code_hash = "${base64sha256(file("lambda_function_payload.zip"))}"
  source_code_hash = base64sha256(data.aws_s3_bucket_object.exampleservice-test2.last_modified)

  runtime = "provided"

  vpc_config {
    # Every subnet should be able to reach an EFS mount target in the same Availability Zone. Cross-AZ mounts are not permitted.
    # subnet_ids         = [data.terraform_remote_state.vpc.outputs.vpc_private_subnet_ids]
    subnet_ids         = ["subnet-02385ede395c1f51a", "subnet-0e6dd749246d5c65d", "subnet-00777fd51e0927323"]
    security_group_ids = ["sg-0f2acf7a4973e5d4c", " "]
  }

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
    aws_cloudwatch_log_group.exampleservice-test2-query-api
  ]
}

# This is to optionally manage the CloudWatch Log Group for the Lambda Function.
# If skipping this resource configuration, also add "logs:CreateLogGroup" to the IAM policy below.
resource "aws_cloudwatch_log_group" "exampleservice-test2-query-api" {
  name              = "/aws/lambda/exampleservice-test2-query-api" # Should be the same as function name
  retention_in_days = 14
}
