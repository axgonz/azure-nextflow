#!/bin/bash

# ubuntu
docker image tag algonz/ubuntu azurecr454.azurecr.io/default/ubuntu
docker push azurecr454.azurecr.io/default/ubuntu

# nextflow
docker image tag algonz/nextflow azurecr454.azurecr.io/default/ubuntu
docker push azurecr454.azurecr.io/default/nextflow