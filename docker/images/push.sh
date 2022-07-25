#!/bin/bash

ACR="azurecr454"

# ubuntu
docker image tag algonz/ubuntu "$ACR.azurecr.io/default/ubuntu"
docker push "$ACR.azurecr.io/default/ubuntu"

# nextflow
docker image tag algonz/nextflow "$ACR.azurecr.io/default/ubuntu"
docker push "$ACR.azurecr.io/default/nextflow"