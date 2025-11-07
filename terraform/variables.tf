variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-east-1"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "prod"
}

variable "app_name" {
  description = "Application name"
  type        = string
  default     = "infinitewebsite"
}

variable "app_port" {
  description = "Port the application runs on"
  type        = number
  default     = 8000
}

variable "lightsail_power" {
  description = "Lightsail container service power (nano, micro, small, medium, large, xlarge)"
  type        = string
  default     = "nano"
}

variable "lightsail_scale" {
  description = "Number of container instances to run"
  type        = number
  default     = 1
}

variable "domain_names" {
  description = "Domain names for SSL certificate (optional)"
  type        = list(string)
  default     = []
}

variable "openai_api_key" {
  description = "OpenAI API key"
  type        = string
  sensitive   = true
}

variable "cloudflare_tunnel_token" {
  description = "Cloudflare Tunnel token"
  type        = string
  sensitive   = true
}