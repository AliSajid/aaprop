# SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
#
# SPDX-License-Identifier: Apache-2.0
# SPDX-License-Identifier: MIT

# This Dockerfile is used to build a Docker image for the aaprop project
# The image is built in two phases:
# 1. The Preparation phase starts fromn the official Rust image to prepare
#    the environment for the final build.
# 2. The Build phase finally builds the binary of the aaprop project.
#    It copies the binary from the builder image to a distroless image and
#    sets it as the entry point of the container
# All in all, this results in the total process of building the Docker image
# consisting of four stages:
# 1. The `chef` stage installs the `cargo-chef` tool in the official Rust image
# 2. The `planner` stage examines the project and builds a minimal recipe for
#    project.
# 3. The `builder` stage uses the recipe from the `planner` stage to build the
#    dependencies and the final binary.
# 4. The `distroless` stage copies the binary from the `builder` stage to distroless
#    image and readies it for production.

# ===================================================================================
# Preparation phase
# ===================================================================================

# -----------------------------------------------------------------------------------
# chef stage
# -----------------------------------------------------------------------------------

# Use the official Rust image as the builder image
# Use the 1.78 version of the Rust image since it's the MSRV (Minimum Supported Rust Version) for the aaprop project
FROM rust:1.78.0@sha256:5907e96b0293eb53bcc8f09b4883d71449808af289862950ede9a0e3cca44ff5 AS chef

# Install the `cargo-chef` tool
RUN cargo install cargo-chef

# -----------------------------------------------------------------------------------
# planner stage
# -----------------------------------------------------------------------------------

# Use the previous stage as the base image
FROM chef AS planner

# Set the working directory to a known location
WORKDIR /usr/src

# Copy the project files to the image
COPY . .

# Use `cargo-chef prepare` to generate a minimal recipe for the project
RUN cargo chef prepare --recipe-path recipe.json

# ===================================================================================
# Build phase
# ===================================================================================

# -----------------------------------------------------------------------------------
# builder stage
# -----------------------------------------------------------------------------------

# Use the previous stage as the base image
FROM chef AS builder

# Set the working directory to a known location
WORKDIR /usr/src

# Copy the `cargo-chef` recipe from the `planner` stage to the current image
COPY --from=planner /usr/src/recipe.json .

# Use `cargo-chef cook` to build the dependencies of the project
RUN cargo chef cook --release --recipe-path recipe.json

# Copy the project files to the image
COPY . .

# Build the Rust project with the actual source code
RUN cargo build --features standalone --no-default-features --release --locked

# -----------------------------------------------------------------------------------
# distroless stage
# -----------------------------------------------------------------------------------

# Use the official distroless image as the base image
FROM gcr.io/distroless/cc-debian12@sha256:e1065a1d58800a7294f74e67c32ec4146d09d6cbe471c1fa7ed456b2d2bf06e0 AS distroless

# Copy the binary from the builder image to the base image
COPY --from=builder /usr/src/target/release/aaprop /usr/local/bin/aaprop

# Change the user to a non-root user for security
USER 1000

# Set the binary as the entry point of the container
# When the container starts, it will execute this binary
ENTRYPOINT [ "/usr/local/bin/aaprop", "--bind", "0.0.0.0", "--port", "80" ]
