FROM ubuntu:20.04

# docker module metadata
LABEL module.name="nxfutil"
LABEL module.version="v0.0.1"

EXPOSE 3000

# Setup OS packages
RUN apt-get update && apt-get install -y \
    libssl-dev \
    curl

# Install Azure Cli
RUN curl -sL https://aka.ms/InstallAzureCLIDeb | bash

# Install handler
RUN mkdir /.nxfutil
ADD handler /.nxfutil/handler
RUN chmod +x /.nxfutil/handler

# Start service pipeline
CMD /.nxfutil/handler