SHELL := /bin/bash
IMAGE := nextflow
ACR   := azurecr454

build:
	echo "----${IMAGE}----" \
	&& pushd ../../../nxfutil/rust/nxfutil \
	&& cargo build --release --target x86_64-unknown-linux-gnu \
	&& popd \
	&& pushd ../../../nxfutil/rust/nxfutild \
	&& cargo build --release --target x86_64-unknown-linux-gnu \
	&& popd \
	&& cp -v ../../../nxfutil/rust/nxfutil/target/x86_64-unknown-linux-gnu/release/nxfutil nxfutil \
	&& cp -v ../../../nxfutil/rust/nxfutild/target/x86_64-unknown-linux-gnu/release/nxfutild nxfutild \
	&& docker build -t "${IMAGE}:serve" -f Dockerfile .

env:
	echo "----${IMAGE}----" \
	&& echo 'NXFUTIL_DISPATCHER="nextflow-20230330-065121-xxxxxxxx-xxxx-xxxx-xxxx-333333333333"' > .env \
	&& echo 'NXFUTIL_API_FQDN="localhost:7071"' >> .env \
	&& echo 'NXFUTIL_AZ_SUB_ID="xxxxxxxx-xxxx-xxxx-xxxx-111111111111"' >> .env \
	&& echo 'NXFUTIL_AZ_RG_NAME="nxfutil004"' >> .env \
	&& echo 'NXFUTIL_AZ_ST_NAME="stxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_CR_NAME="crxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_KV_NAME="kvxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_NAME="nextflowmsi"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_ID="xxxxxxxx-xxxx-xxxx-xxxx-222222222222"' >> .env

enter:
	echo "----${IMAGE}----" \
	&& docker run --env-file .env -it "${IMAGE}:serve" bash

publish:
	echo "----${IMAGE}----" \
	&& az acr login --name "${ACR}" \
	&& docker image tag "${IMAGE}:serve" "${ACR}.azurecr.io/default/${IMAGE}:latest" \
	&& docker push "${ACR}.azurecr.io/default/${IMAGE}:latest"