SHELL := /bin/bash
IMAGE := nxfutil
ACR   := azurecr454

build:
	echo "----${IMAGE}----" \
	&& pushd ../../../azure/functions/rust \
	&& cargo build --release --target x86_64-unknown-linux-gnu \
	&& popd \
	&& cp -v ../../../azure/functions/rust/target/x86_64-unknown-linux-gnu/release/handler handler \
	&& docker build -t "${IMAGE}:serve" -f Dockerfile .

env:
	echo "----${IMAGE}----" \
	&& echo 'NXFUTIL_API_FQDN="localhost:7071"' > .env \
	&& echo 'NXFUTIL_AZ_SUB_ID="xxxxxxxx-xxxx-xxxx-xxxx-111111111111"' >> .env \
	&& echo 'NXFUTIL_AZ_RG_NAME="nxfutil004"' >> .env \
	&& echo 'NXFUTIL_AZ_ST_NAME="stxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_CR_NAME="crxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_KV_NAME="kvxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_NAME="nextflowmsi"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_ID="xxxxxxxx-xxxx-xxxx-xxxx-222222222222"' >> .env

serve:
	echo "----${IMAGE}----" \
	&& docker run --env-file .env -itp 127.0.0.1:7071:3000 "${IMAGE}:serve"

enter:
	echo "----${IMAGE}----" \
	&& docker run --env-file .env -itp 127.0.0.1:7071:3000 "${IMAGE}:serve" bash

publish:
	echo "----${IMAGE}----" \
	&& az acr login --name "${ACR}" \
	&& docker image tag "${IMAGE}:serve" "${ACR}.azurecr.io/default/${IMAGE}:latest" \
	&& docker push "${ACR}.azurecr.io/default/${IMAGE}:latest"