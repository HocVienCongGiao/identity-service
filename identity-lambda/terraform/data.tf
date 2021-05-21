data "aws_api_gateway_rest_api" "query-api" {
  name = "QueryApi"
}

# data "terraform_remote_state" "vpc" {
#   backend = "remote"
#   config = {
#     organization = "HocVienCongGiao"
#     workspaces = {
#       name = "dev-sg-lambda-apps-hvcg-vpc"
#     }
#   }
# }
