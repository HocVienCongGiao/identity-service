# data "terraform_remote_state" "vpc" {
#   backend = "remote"
#   config = {
#     organization = "HocVienCongGiao"
#     workspaces = {
#       name = "dev-sg-lambda-apps-hvcg-vpc"
#     }
#   }
# }
