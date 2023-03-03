FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye
# Install the xz-utils package
RUN apt-get update && apt-get install -y xz-utils


