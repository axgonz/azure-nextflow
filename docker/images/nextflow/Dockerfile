FROM ubuntu 

# docker module metadata
LABEL module.name="nextflow"
LABEL module.version="v0.0.1"

# Set nextflow variables
ENV NXF_VER=22.04.4
ENV NXF_HOME=/.nextflow
ENV NXF_ENABLE_SECRETS=true

# Set Azure variables
ENV AZ_KEY_VAULT_NAME="kv718"

# Setup OS packages
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    curl \
    default-jre \
    git \
    python3 \
    pip

# Add build dependacies
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y \
    gradle

# Build nextflow from source
RUN mkdir ~/repo 
    && cd ~/repo
    && git clone --branch STABLE-22.10.x https://github.com/nextflow-io/nextflow.git
    && cd nextflow
    && make compile
    && make pack

# Install nextflow
RUN mkdir /.nextflow \
    && cd /.nextflow \
    && cp ~/repo/nextflow/nextflow ./
    && chmod +x nextflow

# Install python modules
RUN pip install requests \
    && pip install argparse \
    && pip install azure-identity \
    && pip install azure-keyvault-secrets

# Install nxfutil
ADD nxfutil.py /.nextflow/nxfutil.py
ADD nxfutil /.nextflow/nxfutil
RUN cd /.nextflow \
    && chmod +x ./nxfutil

# Run pipeline
CMD cd /.nextflow \
    && ./nxfutil