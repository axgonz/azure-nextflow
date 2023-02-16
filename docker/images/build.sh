#!/bin/bash

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd ${script_dir}

echo "---ubuntu---"
cd ubuntu
docker build -t nxfutil/ubuntu -f Dockerfile .
cd ..

echo "---nextflow---"
cp ../../nxfutil/rust/nxfutil/target/release/nxfutil nextflow/
cp ../../nxfutil/rust/nxfutild/target/release/nxfutild nextflow/
cd nextflow
docker build -t nxfutil/nextflow -f Dockerfile .
cd ..
