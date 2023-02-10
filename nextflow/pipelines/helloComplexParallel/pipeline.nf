#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process parallel {
    cpus 2
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val data

    output:
        stdout

    script:
        """
        echo "Generate ref for $params.name"
        echo "Fitting ${data} on ${task.cpus} cpus"
        """
}

workflow {
    Channel.from(0..16) | parallel | view
}
