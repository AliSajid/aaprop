<!--
SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami

SPDX-License-Identifier: Apache-2.0
SPDX-License-Identifier: MIT
-->

# Deployment Guide

You can deploy the `AAProp` API in two distinct ways:

1. Self-Hosted Deployment using the binaries provided in the [releases](https://gtihub.com/alisajid/aaprop/releases) section.
2. Docker-based Deployment using the Docker image provided in the [Docker Hub](https://hub.docker.com/r/imamiland/aaprop) or the [GitHub Container Registry](https://ghcr.io/alisajid/aaprop).

## Self-Hosted Deployment

### Prerequisites

1. One of the following architectures:
   - `x86_64`
   - `aarch64`
   - `i686`
2. A Linux-based operating system, preferably:
   - Ubuntu 20.04 LTS or later
   - Fedora 34 or later
   - CentOS 8 or later
   - Debian 11 or later
3. At least of 128 MB of RAM.
4. The `AAProp` binary for your architecture.
5. A reverse proxy like `nginx` or `Caddy` to handle incoming requests, if you want to expose the API to the internet.

### Installation

1. Download the `AAProp` binary for your architecture from the [releases](https://github.com/alisajid/aaprop/releases/latest) section.

    ```bash
        wget "https://github.com/AliSajid/aaprop/releases/download/v2.0.0/aaprop-{`arch`}-unknown-linux-gnu"
        wget "https://github.com/AliSajid/aaprop/releases/download/v2.0.0/aaprop-{`arch`}-unknown-linux-gnu.asc"
        wget "https://github.com/AliSajid/aaprop/releases/download/v2.0.0/SHA256SUMS.txt"
        wget "https://github.com/AliSajid/aaprop/releases/download/v2.0.0/SHA256SUMS.txt.asc"
    ```

1. Verify the integrity of the binary using the provided [SHA256 checksum](https://github.com/AliSajid/aaprop/releases/download/v2.0.0/SHA256SUMS.txt).

    ```bash
        sha256sum -c SHA256SUMS.txt 2>&1 | grep OK
    ```

1. Verify the binary's authenticity by checking the gpg signature.

    ```bash
        gpg --verify SHA256SUMS.txt.asc
        gpg --verify aaprop-$(arch)-unknown-linux-gnu.asc
    ```

1. Place the binary in a directory of your choice in the PATH.

    ```bash
        sudo mv aaprop-$(arch)-unknown-linux-gnu /usr/local/bin/aaprop
    ```

1. Start the `AAProp` API.

    ```bash
        aaprop
    ```


## Docker Deployment

### Prerequisites

1. Docker installed on your system.
2. A reverse proxy like `nginx` or `Caddy` to handle incoming requests, if you want to expose the API to the internet.
3. The Docker image for the `AAProp` API.

### Installation

1. Pull the Docker image from the [Docker Hub](https://hub.docker.com/r/imamiland/aaprop) or the [GitHub Container Registry](https://ghcr.io/alisajid/aaprop).

    ```bash
        docker pull imamiland/aaprop:latest
    ```

2. Run the Docker container.

    ```bash
        docker run -d -p 8080:8080 imamiland/aaprop:latest
    ```

3. Verify that the container is running.

    ```bash
        docker ps
    ```

4. Access the API at `http://localhost:8080`.
5. If you want to expose the API to the internet, configure your reverse proxy to forward requests to the container's port.

    ```nginx
        server {
            listen 80;
            server_name example.com;

            location / {
                proxy_pass http://localhost:8080;
            }
        }
    ```

    ```bash
        sudo systemctl restart nginx
    ```
