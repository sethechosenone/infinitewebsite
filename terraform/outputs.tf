output "lightsail_service_name" {
  description = "Name of the Lightsail container service"
  value       = aws_lightsail_container_service.app.name
}

output "lightsail_url" {
  description = "Public URL of the Lightsail container service"
  value       = aws_lightsail_container_service.app.url
}

output "lightsail_state" {
  description = "State of the Lightsail container service"
  value       = aws_lightsail_container_service.app.state
}

output "ecr_repository_url" {
  description = "URL of the ECR repository"
  value       = aws_ecr_repository.app.repository_url
}