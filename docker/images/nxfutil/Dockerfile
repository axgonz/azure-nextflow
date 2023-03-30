FROM ubuntu:latest

# docker module metadata
LABEL module.name="nxfutil"
LABEL module.version="v0.0.1"

EXPOSE 3000

# Setup OS packages
RUN apt-get update && apt-get install -y \
    libssl-dev \
    curl

# Update certificates
RUN apt-get install ca-certificates -y \
    && update-ca-certificates

# Install handler
RUN mkdir /.nxfutil
ADD handler /.nxfutil/handler
RUN chmod +x /.nxfutil/handler

# Start service pipeline
CMD /.nxfutil/handler 