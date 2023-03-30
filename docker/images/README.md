# Docker images

## Pre-requisites

### Tool chain

Building these containers requires a ubuntu (or similar) build host. This is to avoid complexities of cross-compilation.

When developing locally using a Windows system this can be achieved by enabling WSL and then installing the Rust and Azure Functions tool chains.

The Docker Engine is also required to build these images.

- [Install Linux on Windows with WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install the Azure CLI on Linux](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli-linux?pivots=apt)
- [Install Docker Engine](https://docs.docker.com/engine/install/ubuntu/)

These pre-requisites are met when the following commands return without errors in the WSL terminal:

```
cargo --version;
az --version;
docker --version;
```

> NOTE: When building via a CI-CD pipeline be sure to select an appropriate linux build host.

### Access requirements

This app creates a Container Instance in Azure using an image from a Container Registry and pulls secrets from Key Vault to do so.

Ensure when developing locally that the account being used has the following access:

- Contributor on the target Resource Group.
- Arc Pull on the Container Registry.
- Equivalent of "secret list" and "secret read" permissions within Key Vault.

## Setup

When developing locally:

- Launch VS Code from this directory.
- In VS Code use WSL:Ubuntu as the terminal.
- Ensure to cache an access token locally (for DefaultAzureCredential to pick up), do this using the Azure CLI:

    ```
    az login;
    ```

A Makefile is provided with each image to aid the developer. Each Makefile can be used to perform the steps written out in full below.

> IMPORTANT: Ensure to run the Makefile in the subfolder of the desired image.

**`make`**

Builds any pre-requisite rust binaries, and copies them locally and build the docker image.

**`make env`**

If the image requires certain environment variables, this will create a template `.env` file which can be updated manually *before* running any of the below commands.

**`make serve`**

If the image contains a http server, this will start the container and expose its ports.

**`make enter`**

Used for troubleshooting the image, this will override any start CMD with an interactive Bash shell in the container.
