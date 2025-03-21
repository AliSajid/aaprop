<!--
SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

<!-- vale Google.Passive = NO -->
# **AAProp API Deployment Guide**  

This document provides a comprehensive guide for deploying the **AAProp** API using different methods. The API can be deployed in the following ways:  

1. **Self-Hosted Deployment** – Utilizing precompiled binaries available in the [GitHub Releases](https://github.com/alisajid/aaprop/releases) section.  
2. **Docker-Based Deployment** – Running the API within a containerized environment using images from [Docker Hub](https://hub.docker.com/r/imamiland/aaprop) or the [GitHub Container Registry](https://ghcr.io/alisajid/aaprop).  
3. **Docker-Compose Deployment** – Deploying the API alongside a reverse proxy and/or database using `docker-compose`.  

## **1. Self-Hosted Deployment**  

### **1.1 Prerequisites**  

Before proceeding with a self-hosted installation, ensure the following prerequisites are met:  

1. **Supported Architecture:**  
   - `x86_64`  
   - `aarch64`  
   - `i686`  

2. **Supported Operating Systems:**  
   - Ubuntu 20.04 LTS or later  
   - Fedora 34 or later  
   - CentOS 8 or later  
   - Debian 11 or later  

3. **Minimum Hardware Requirements:**  
   - At least **128 MB of RAM**  
   - At least **20 MB of free disk space**  

4. **Required Software:**  
   - The `AAProp` binary compatible with your system architecture  
   - (Optional) A reverse proxy such as **Nginx** or **Caddy** for handling external requests  

### **1.2 Installation Steps**  

#### **Step 1: Download the Precompiled Binary**  

Download the latest `AAProp` binary and its associated verification files from the **GitHub Releases** page:  

```bash
wget "https://github.com/AliSajid/aaprop/releases/download/v2.1.1/aaprop-$(arch)-unknown-linux-gnu"
wget "https://github.com/AliSajid/aaprop/releases/download/v2.1.1/aaprop-$(arch)-unknown-linux-gnu.asc"
wget "https://github.com/AliSajid/aaprop/releases/download/v2.1.1/SHA256SUMS.txt"
wget "https://github.com/AliSajid/aaprop/releases/download/v2.1.1/SHA256SUMS.txt.asc"
```  

#### **Step 2: Verify Integrity and Authenticity**  

Verify the SHA256 checksum to ensure file integrity:  

```bash
sha256sum -c SHA256SUMS.txt 2>&1 | grep OK
```  

Authenticate the binary using **GPG signature verification**:  

```bash
gpg --verify SHA256SUMS.txt.asc
gpg --verify aaprop-$(arch)-unknown-linux-gnu.asc
```  

#### **Step 3: Install the Binary**  

Move the binary to a directory within the system's `$PATH`:  

```bash
sudo mv aaprop-$(arch)-unknown-linux-gnu /usr/local/bin/aaprop
chmod +x /usr/local/bin/aaprop
```  

#### **Step 4: Start the API Server**  

Run the `AAProp` API:  

```bash
aaprop
```  

---

## **2. Docker Deployment**  

### **2.1 Prerequisites**  

Ensure the following requirements are met before deploying via Docker:  

1. **Docker Installed**  
   - Install Docker from [official documentation](https://docs.docker.com/get-docker/).  

2. **Pull the `AAProp` Docker Image**  
   - Available on **Docker Hub** or **GitHub Container Registry**  

3. **(Optional) Reverse Proxy**  
   - **Nginx** or **Caddy** to expose the API publicly  

### **2.2 Deployment Steps**  

#### **Step 1: Pull the Docker Image**  

```bash
docker pull imamiland/aaprop:latest
```  

#### **Step 2: Run the API as a Container**  

```bash
docker run -d -p 8080:8080 --name aaprop imamiland/aaprop:latest
```  

#### **Step 3: Verify the Running Container**  

```bash
docker ps
```  

#### **Step 4: Access the API**  

Once the container is running, the API is accessible at:  

```bash
http://localhost:8080
```  

#### **Step 5: Configuring a Reverse Proxy (Optional)**  

If you wish to expose the API publicly, configure an **Nginx** reverse proxy:  

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```  

Restart **Nginx** to apply changes:  

```bash
sudo systemctl restart nginx
```  

---

## **3. Docker-Compose Deployment**  

For **multi-container** deployments, `docker-compose` simplifies managing services such as the **AAProp API** and a **reverse proxy**.  

### **3.1 Prerequisites**  

Ensure the following:  

- **Docker** and **docker-compose** are installed  
- A valid `docker-compose.yml` file  

### **3.2 Deployment Steps**  

#### **Step 1: Create a `docker-compose.yml` File**  

```yaml
version: '3.8'

services:
  aaprop:
    image: imamiland/aaprop:latest
    container_name: aaprop
    restart: always
    ports:
      - "8080:8080"

  nginx:
    image: nginx:latest
    container_name: reverse_proxy
    restart: always
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
    ports:
      - "80:80"
    depends_on:
      - aaprop
```

#### **Step 2: Create an `nginx.conf` File**  

```nginx
events {}

http {
    server {
        listen 80;
        server_name example.com;

        location / {
            proxy_pass http://aaprop:8080;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        }
    }
}
```  

#### **Step 3: Deploy the Stack**  

Run the following command to start both **AAProp** and **Nginx**:  

```bash
docker-compose up -d
```  

#### **Step 4: Verify Deployment**  

Check that the services are running:

```bash
docker-compose ps
```  

---

## **4. Conclusion**  

This guide provides three distinct deployment methods for the **AAProp** API:  

1. **Self-Hosting** – Suitable for lightweight deployments on personal servers.  
2. **Docker Deployment** – Ideal for running the API in an isolated, containerized environment.  
3. **Docker-Compose Deployment** – A more scalable approach allowing multi-container orchestration with a reverse proxy.  

Depending on your infrastructure requirements, choose the most suitable method. If deploying in production, consider **securing your API with HTTPS** using **Let's Encrypt** with **Nginx** or **Caddy**.  

For troubleshooting or further optimizations, refer to the [official repository](https://github.com/alisajid/aaprop). 🚀  

<!-- vale Google.Passive = YES -->
