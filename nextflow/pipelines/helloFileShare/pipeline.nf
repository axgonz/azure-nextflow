#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    output:
        stdout

    shell:
        """
        !{params.azureFileShare}/script.sh
        """
}

workflow {
    listFiles | view
}