# Official Microsoft Dev Container Rust Image
# FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

# Official Rust Organization Image
FROM rust:1.71.0-bookworm

ARG USERNAME=rust_user
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt update \
    && apt install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# Install the xz-utils package
RUN apt update && apt install -y xz-utils

# # [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME

RUN rustup component add rustfmt


