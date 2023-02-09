#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    script:
        """
        ls -la ${params.azureFileShare}
        """
}

workflow {
    listFiles
}
