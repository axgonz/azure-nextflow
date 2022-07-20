#!/bin/bash

# ubuntu
docker image tag ubuntu azurecr454.azurecr.io/default/ubuntu
docker push azurecr454.azurecr.io/default/ubuntu

# nextflow
docker image tag nextflow azurecr454.azurecr.io/default/ubuntu
docker push azurecr454.azurecr.io/default/nextflow