#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process fitData {
    cpus 2
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val data

    output:
        stdout

    script:
        """
        sleep 30s
        echo "${data} on ${task.cpus} cpus"
        """
}

workflow {
    Channel.fromList(params.tasks) | fitData | view
}
