#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azure_registry_server/default/ubuntu:latest"

    output:
        stdout

    script:
        """
        ${params.azure_file_share}/script.sh >> ${params.azure_file_share}/script.log
        cp results.log ${params.azure_file_share}/results.log
        """
}

workflow {
    listFiles | view
}