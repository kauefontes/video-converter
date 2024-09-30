provider "google" {
  project = var.project_id
  region  = var.region
}

variable "project_id" {
  description = "ID do projeto do Google Cloud"
  type        = string
  default     = "mobilizasp-64f26"
}

variable "region" {
  description = "Região do Google Cloud"
  type        = string
  default     = "us-central1"
}

data "google_secret_manager_secret_version" "github_oauth_token" {
  secret  = "github_oauth_token_kaue"
  project = var.project_id
  version = "latest"
}

resource "google_cloud_run_service" "video_converter" {
  name     = "video-converter"
  location = var.region

  template {
    spec {
      containers {
        image = "gcr.io/${var.project_id}/video-converter:latest"
        ports {
          container_port = 8080
        }
        resources {
          limits = {
            memory = "512Mi"
            cpu    = "1"
          }
        }
        env {
          name  = "GITHUB_OAUTH_TOKEN"
          value = data.google_secret_manager_secret_version.github_oauth_token.secret_data
        }
      }
    }
  }

  traffic {
    percent         = 100
    latest_revision = true
  }
}

resource "google_cloud_run_service_iam_member" "noauth" {
  service  = google_cloud_run_service.video_converter.name
  location = google_cloud_run_service.video_converter.location
  role     = "roles/run.invoker"
  member   = "allUsers"
}

output "url" {
  value = google_cloud_run_service.video_converter.status[0].url
}

