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

variable "github_repo_name" {
  description = "Nome do repositório no GitHub"
  type        = string
  default     = "kauefontes/video-converter"
}

variable "github_oauth_token" {
  description = "Token OAuth do GitHub"
  type        = string
  sensitive   = true
}

resource "google_cloudbuild_trigger" "docker_image_build" {
  name = "docker-image-build"

  github {
    owner = split("/", var.github_repo_name)[0]
    name  = split("/", var.github_repo_name)[1]
    push {
      branch = "main"
    }
  }

  build {
    step {
      name = "gcr.io/cloud-builders/docker"
      args = ["build", "-t", "gcr.io/${var.project_id}/video-converter:latest", "."]
    }

    step {
      name = "gcr.io/cloud-builders/docker"
      args = ["push", "gcr.io/${var.project_id}/video-converter:latest"]
    }

    images = ["gcr.io/${var.project_id}/video-converter:latest"]
  }
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