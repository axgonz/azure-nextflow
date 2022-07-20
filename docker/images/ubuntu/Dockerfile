FROM ubuntu

# docker module metadata
LABEL module.name="ubuntu"
LABEL module.version="v0.0.1"

# Setup OS packages
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    curl \
    default-jre \
    git