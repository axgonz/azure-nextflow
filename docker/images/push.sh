#!/bin/bash

ACR="azurecr454"

# ubuntu
docker image tag nxfutil/ubuntu "$ACR.azurecr.io/default/ubuntu"
docker push "$ACR.azurecr.io/default/ubuntu"

# nextflow
docker image tag nxfutil/nextflow "$ACR.azurecr.io/default/nextflow"
docker push "$ACR.azurecr.io/default/nextflow"