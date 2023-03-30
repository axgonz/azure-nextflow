SHELL := /bin/bash
IMAGE := ubuntu
ACR   := azurecr454

build:
	echo "----${IMAGE}----" \
	&& docker build -t "${IMAGE}:serve" -f Dockerfile .

enter:
	echo "----${IMAGE}----" \
	&& docker run -it "${IMAGE}:serve" bash

publish:
	echo "----${IMAGE}----" \
	&& az acr login --name "${ACR}" \
	&& docker image tag "${IMAGE}:serve" "${ACR}.azurecr.io/default/${IMAGE}:latest" \
	&& docker push "${ACR}.azurecr.io/default/${IMAGE}:latest"