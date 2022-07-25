#!/bin/bash

ACR="azurecr454"

az login
az acr login --name $ACR