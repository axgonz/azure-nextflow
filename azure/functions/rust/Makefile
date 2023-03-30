SHELL := /bin/bash

build:
	echo "----Azure Function----" \
	&& cargo build --release --target x86_64-unknown-linux-gnu \
	&& cp -v target/x86_64-unknown-linux-gnu/release/handler handler \

env:
	echo "----Azure Function----" \
	&& echo '{' > local.settings.json \
	&& echo '    "IsEncrypted": false,' >> local.settings.json \
	&& echo '    "Values": {' >> local.settings.json \
	&& echo '        "AzureWebJobsStorage": "",' >> local.settings.json \
	&& echo '        "FUNCTIONS_WORKER_RUNTIME": "custom",' >> local.settings.json \
	&& echo '        "NXFUTIL_API_FQDN": "localhost:7071",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_SUB_ID": "xxxxxxxx-xxxx-xxxx-xxxx-111111111111",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_RG_NAME": "nxfutil004",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_ST_NAME": "stxxxx",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_CR_NAME": "crxxxx",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_KV_NAME": "kvxxxx",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_MSI_NAME": "nextflowmsi",' >> local.settings.json \
	&& echo '        "NXFUTIL_AZ_MSI_ID": "xxxxxxxx-xxxx-xxxx-xxxx-222222222222"' >> local.settings.json \
	&& echo '    },' >> local.settings.json \
	&& echo '    "Host": {' >> local.settings.json \
	&& echo '        "LocalHttpPort": 7071,' >> local.settings.json \
	&& echo '        "CORS": "*"' >> local.settings.json \
	&& echo '    }' >> local.settings.json \
	&& echo '}' >> local.settings.json

	echo 'NXFUTIL_API_FQDN="localhost:7071"' > .env \
	&& echo 'NXFUTIL_AZ_SUB_ID="xxxxxxxx-xxxx-xxxx-xxxx-111111111111"' >> .env \
	&& echo 'NXFUTIL_AZ_RG_NAME="nxfutil004"' >> .env \
	&& echo 'NXFUTIL_AZ_ST_NAME="stxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_CR_NAME="crxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_KV_NAME="kvxxxx"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_NAME="nextflowmsi"' >> .env \
	&& echo 'NXFUTIL_AZ_MSI_ID="xxxxxxxx-xxxx-xxxx-xxxx-222222222222"' >> .env

serve:
	echo "----Azure Function----" \
	&& func start
