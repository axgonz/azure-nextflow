#!/bin/bash

build:
	cd ../../../azure/functions/rust \
	&& cargo build --release --target x86_64-unknown-linux-gnu \
	&& cp -v target/x86_64-unknown-linux-gnu/release/handler handler

serve:
	cp -v ../../../azure/functions/rust/handler handler \
	&& docker build -t nxfutil:serve -f Dockerfile . \
	&& docker run --env-file .env -itp 127.0.0.1:7071:3000 nxfutil:serve

enter:
	cp -v ../../../azure/functions/rust/handler handler \
	&& docker build -t nxfutil:serve -f Dockerfile . \
	&& docker run --env-file .env -itp 127.0.0.1:7071:3000 nxfutil:serve bash