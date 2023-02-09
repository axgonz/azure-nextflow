#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    output:
        path 'result.log'

    script:
        """
        ls -la ${params.azureFileShare} >> result.log
        """
}

workflow {
    listFiles | view { it.trim() }
}
