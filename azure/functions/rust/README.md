# Azure Function App

## Pre-requisites

### Tool chain

Building this app requires a ubuntu (or similar) build host. This is to avoid complexities of cross-compilation.

When developing locally using a Windows system this can be achieved by enabling WSL and then installing the Rust and Azure Functions tool chains.

- [Install Linux on Windows with WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
- [Install Rust](https://www.rust-lang.org/tools/install)
- [Install the Azure CLI on Linux](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli-linux?pivots=apt)
- [Install the Azure Functions Core Tools](https://learn.microsoft.com/en-us/azure/azure-functions/functions-run-local?tabs=v4%2Clinux%2Ccsharp%2Cportal%2Cbash)

These pre-requisites are met when the following commands return without errors in the WSL terminal:

```
cargo --version;
az --version;
func --version;
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

A Makefile is provided to aid the developer. This Makefile can be used to perform the steps written out in full below. The Makefile command is provided above each step.

**`make env`**

Creates a `local.settings.json` file and `.env` file in the same directory as this README.

Update variables that start with `NXFUTIL_` in the `local.settings.json` and `.env` file before continuing.

> IMPORTANT: Do not provide the `AZURE_CLIENT_ID` value when developing locally, this is used for Azure Manage Identities.

**`make`**

Compiles the rust binary and copies the binary to the same directory as this README.

    ```
    cargo build --target x86_64-unknown-linux-gnu;
    cp -v target/release/handler handler;
    ```

**`make serve`**

Runs the function app locally.

    ```
    func start;
    ```

> IMPORTANT: When ready to deploy to production build the release version of the rust binary by adding `--release` to the cargo build command.
