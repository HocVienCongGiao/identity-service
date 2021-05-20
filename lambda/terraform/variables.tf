variable "service_name" {}

variable "aws_region" {
  type    = string
  default = "ap-southeast-1"
}

variable "aws_access_key_id" {}
variable "aws_secret_access_key" {}


variable "aws_account_id" {
   type = string 
 }

variable "api_key" {
  type    = string
  default = "DEFAULT_API_KEY_FOR_NOTIF"
}

variable "tfe_token" {
  type = string
}
