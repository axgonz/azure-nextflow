#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    output:
        stdout

    script:
        """
        ${params.azureFileShare}/script.sh >> ${params.azureFileShare}/script.log
        cp results.log ${params.azureFileShare}/results.log
        """
}

workflow {
    listFiles | view
}