#!/bin/bash

script_dir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd ${script_dir}

echo "---ubuntu---"
cd ubuntu
docker build -t algonz/ubuntu -f Dockerfile .
cd ..

echo "---nextflow---"
cd nextflow
docker build -t algonz/nextflow -f Dockerfile .
cd ..
