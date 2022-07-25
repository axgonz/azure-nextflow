#!/bin/bash

ACR="azurecr454"

az login --tenant microsoft.com
az acr login --name $ACR