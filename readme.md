# Video Converter

This project is a video and image processing service built with Rust and Axum. It provides endpoints to upload videos and images, convert videos to Apple-compatible formats, and optimize images.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Running Locally](#running-locally)
- [Deploying to Google Cloud Run](#deploying-to-google-cloud-run)
- [API Endpoints](#api-endpoints)
- [Integrating with Frontend](#integrating-with-frontend)
- [License](#license)

## Prerequisites

- Docker
- Docker Compose
- Google Cloud SDK (for deployment)
- Rust (for local development)

## Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/kauefontes/video-converter.git
    cd video-converter
    ```

2. Build the Docker image:
    ```sh
    docker-compose build
    ```

## Running Locally

1. Start the Docker container:
    ```sh
    docker-compose up
    ```

2. The service will be available at `http://localhost:8080`.

## Deploying to Google Cloud Run

1. Authenticate with Google Cloud:
    ```sh
    gcloud auth login
    gcloud config set project YOUR_PROJECT_ID
    ```

2. Build and push the Docker image to Google Container Registry:
    ```sh
    docker build -t gcr.io/YOUR_PROJECT_ID/video-converter:latest .
    docker push gcr.io/YOUR_PROJECT_ID/video-converter:latest
    ```

3. Deploy to Google Cloud Run:
    ```sh
    terraform init
    terraform apply
    ```

4. The service URL will be output by Terraform.

## API Endpoints

### Upload Video

- **URL:** `/upload_video`
- **Method:** `POST`
- **Content-Type:** `multipart/form-data`
- **Parameters:**
    - `file`: The video file to be uploaded.
    - `presigned_url`: The presigned URL to upload the converted video to S3.

### Upload Image

- **URL:** `/upload_image`
- **Method:** `POST`
- **Content-Type:** `multipart/form-data`
- **Parameters:**
    - `file`: The image file to be uploaded.
    - `presigned_url`: The presigned URL to upload the optimized image to S3.

## Integrating with Frontend

Here is an example of how to integrate the endpoints with a frontend using JavaScript and HTML.

### HTML

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Video Converter</title>
</head>
<body>
    <h1>Upload Video</h1>
    <form id="videoForm">
        <input type="file" id="videoFile" name="file" accept="video/*" required>
        <input type="text" id="videoPresignedUrl" name="presigned_url" placeholder="Presigned URL" required>
        <button type="submit">Upload Video</button>
    </form>

    <h1>Upload Image</h1>
    <form id="imageForm">
        <input type="file" id="imageFile" name="file" accept="image/*" required>
        <input type="text" id="imagePresignedUrl" name="presigned_url" placeholder="Presigned URL" required>
        <button type="submit">Upload Image</button>
    </form>

    <script src="app.js"></script>
</body>
</html>
```

### JavaScript

```javascript
document.getElementById('videoForm').addEventListener('submit', async (event) => {
    event.preventDefault();

    const file = document.getElementById('videoFile').files[0];
    const presignedUrl = document.getElementById('videoPresignedUrl').value;

    const formData = new FormData();
    formData.append('file', file);
    formData.append('presigned_url', presignedUrl);

    const response = await fetch('http://localhost:8080/upload_video', {
        method: 'POST',
        body: formData,
    });

    if (response.ok) {
        alert('Video uploaded successfully!');
    } else {
        alert('Failed to upload video.');
    }
});

document.getElementById('imageForm').addEventListener('submit', async (event) => {
    event.preventDefault();

    const file = document.getElementById('imageFile').files[0];
    const presignedUrl = document.getElementById('imagePresignedUrl').value;

    const formData = new FormData();
    formData.append('file', file);
    formData.append('presigned_url', presignedUrl);

    const response = await fetch('http://localhost:8080/upload_image', {
        method: 'POST',
        body: formData,
    });

    if (response.ok) {
        alert('Image uploaded successfully!');
    } else {
        alert('Failed to upload image.');
    }
});
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.