terraform {
  required_version = ">= 1.0"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 5.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
}

# Data sources
data "aws_availability_zones" "available" {
  state = "available"
}

# Cloudflare IP ranges for security group restrictions
data "http" "cloudflare_ips_v4" {
  url = "https://www.cloudflare.com/ips-v4"
}

locals {
  cloudflare_ips = split("\n", trimspace(data.http.cloudflare_ips_v4.response_body))
  
  common_tags = {
    Project     = "infinitewebsite"
    Environment = var.environment
    ManagedBy   = "terraform"
  }
}