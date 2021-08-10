module "users" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.identity-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.identity-service.mutation_api_gateway_resource_id
    
  function_name = "users"
  file_name     = "user"

  depends_on = [
    module.identity-service
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "users_id" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.users.query_api_gateway_resource_id
  mutation_api_parent_id = module.users.mutation_api_gateway_resource_id
    
  function_name = "users_id"
  file_name     = "user"
  path_part     = "{id}"
  depends_on = [
    module.users
   ]
    
  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "user" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.identity-service.query_api_gateway_resource_id
  mutation_api_parent_id = module.identity-service.mutation_api_gateway_resource_id

  function_name = "user"
  is_query_api  = false

  depends_on = [
    module.identity-service
  ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "users_deactivation" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.users.query_api_gateway_resource_id
  mutation_api_parent_id = module.users.mutation_api_gateway_resource_id

  function_name = "users_deactivation"
  file_name     = "user"
  path_part     = "deactivation"
  depends_on = [
    module.users
  ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "users_activation" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.users.query_api_gateway_resource_id
  mutation_api_parent_id = module.users.mutation_api_gateway_resource_id

  function_name = "users_activation"
  file_name     = "user"
  path_part     = "activation"
  depends_on = [
    module.users
  ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}

module "users_update_password" {
  source = "git::ssh://git@github.com/HocVienCongGiao/terraform-infra.git//skeleton/services/service-function"
  service_name = var.service_name
  query_api_parent_id = module.users.query_api_gateway_resource_id
  mutation_api_parent_id = module.users.mutation_api_gateway_resource_id

  function_name = "users_update_password"
  file_name     = "user"
  path_part     = "{id}/password"
  depends_on = [
    module.users
  ]

  environment = var.environment
  db_host              = var.db_host
  db_user              = var.db_user
  db_password          = var.db_password
  db_name              = var.db_name
}