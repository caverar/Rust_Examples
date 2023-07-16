FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm
# Install the xz-utils package
RUN apt-get update && apt-get install -y xz-utils


