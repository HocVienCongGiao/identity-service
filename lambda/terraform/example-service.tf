/*
The "REST API" is the container for all of the other API Gateway objects we will create.
All incoming requests to API Gateway must match with a configured resource and method in order to be handled. 
Append the following to define a single proxy resource:
*/
resource "aws_api_gateway_resource" "exampleservice-query-api-proxy" {
  rest_api_id = data.aws_api_gateway_rest_api.query-api.id
  parent_id   = data.aws_api_gateway_rest_api.query-api.root_resource_id
  path_part   = "example-service"
}
resource "aws_api_gateway_method" "exampleservice-query-api-proxy" {
  rest_api_id   = data.aws_api_gateway_rest_api.query-api.id
  resource_id   = aws_api_gateway_resource.exampleservice-query-api-proxy.id
  http_method   = "ANY"
  authorization = "NONE"
}
/*
The special path_part value "{proxy+}" activates proxy behavior, which means that this resource will match any request path. 
Similarly, the aws_api_gateway_method block uses a http_method of "ANY", which allows any request method to be used. 
Taken together, this means that all incoming requests will match this resource.
Each method on an API gateway resource has an integration which specifies where incoming requests are routed. 
Add the following configuration to specify that requests to this method should be sent to the Lambda function defined earlier:
*/
resource "aws_api_gateway_integration" "exampleservice-query-api" {
  rest_api_id = data.aws_api_gateway_rest_api.query-api.id
  resource_id = aws_api_gateway_method.exampleservice-query-api-proxy.resource_id
  http_method = aws_api_gateway_method.exampleservice-query-api-proxy.http_method

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.exampleservice-test1-query-api.invoke_arn
}


/*
After the creation steps are complete, the new objects will be visible in the API Gateway console.
The integration with the Lambda function is not functional yet
because API Gateway does not have the necessary access to invoke the function.
The next step will address this, making the application fully-functional.

»Allowing API Gateway to Access Lambda
By default any two AWS services have no access to one another,
until access is explicitly granted. For Lambda functions, access is granted using the aws_lambda_permission resource,
which should be added to the lambda.tf file created in an earlier step:
*/
resource "aws_lambda_permission" "exampleservice-query-api-gateway" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.exampleservice-test1-query-api.function_name
  principal     = "apigateway.amazonaws.com"

  # The "/*/*" portion grants access from any method on any resource
  # within the API Gateway REST API.
  source_arn = "${data.aws_api_gateway_rest_api.query-api.execution_arn}/*/*"
}




# https://learn.hashicorp.com/tutorials/terraform/lambda-api-gateway?in=terraform/aws
/*
Each Lambda function must have an associated IAM role which dictates what access it has to other AWS services. 
*/
resource "aws_iam_role" "iam_for_lambda" {
  name = "iam_for_lambda_example-service"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": 
 "sts:AssumeRole"
      ,
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

# See also the following AWS managed policy: AWSLambdaBasicExecutionRole
resource "aws_iam_policy" "lambda_logging" {
  name        = "lambda_logging_example-service"
  path        = "/"
  description = "IAM policy for logging from a lambda"

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "logs:CreateLogGroup",
        "logs:CreateLogStream",
        "logs:PutLogEvents"
      ],
      "Resource": "arn:aws:logs:*:*:*",
      "Effect": "Allow"
    }
  ]
}
EOF
}

resource "aws_iam_role_policy_attachment" "lambda_logs" {
  role       = aws_iam_role.iam_for_lambda.name
  policy_arn = aws_iam_policy.lambda_logging.arn
}


