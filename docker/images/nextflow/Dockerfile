FROM ubuntu 

# docker module metadata
LABEL module.name="nextflow"
LABEL module.version="v0.0.1"

# Set nextflow variables
ENV NXF_VER=22.10.6
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

# Install nextflow
RUN mkdir /.nextflow \
    && cd /.nextflow \
    && curl -s https://get.nextflow.io | bash \
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