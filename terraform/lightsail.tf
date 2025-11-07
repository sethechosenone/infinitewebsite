# Lightsail Container Service
resource "aws_lightsail_container_service" "app" {
  name  = var.app_name
  power = var.lightsail_power
  scale = var.lightsail_scale

  # Only configure domain names if provided
  dynamic "public_domain_names" {
    for_each = length(var.domain_names) > 0 ? [1] : []
    content {
      certificate {
        certificate_name = var.app_name
        domain_names     = var.domain_names
      }
    }
  }

  tags = merge(local.common_tags, {
    Name = "${var.app_name}-lightsail"
  })
}

# Container Service Deployment
resource "aws_lightsail_container_service_deployment_version" "app" {
  service_name = aws_lightsail_container_service.app.name

  container {
    container_name = var.app_name
    image          = "${aws_ecr_repository.app.repository_url}:latest"

    environment = {
      RUST_LOG                = "info"
      ROCKET_ADDRESS          = "0.0.0.0"
      ROCKET_PORT             = tostring(var.app_port)
      OPENAI_KEY              = var.openai_api_key
      CLOUDFLARE_TUNNEL_TOKEN = var.cloudflare_tunnel_token
    }

    ports = {
      (var.app_port) = "HTTP"
    }
  }

}

# SSM Parameter for OpenAI API Key (keep for environment variable)
resource "aws_ssm_parameter" "openai_api_key" {
  name  = "/${var.app_name}/openai_api_key"
  type  = "SecureString"
  value = var.openai_api_key

  tags = merge(local.common_tags, {
    Name = "${var.app_name}-openai-api-key"
  })
}