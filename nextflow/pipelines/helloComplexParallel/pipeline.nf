#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process fitData {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val data

    output:
        stdout

    script:
        """       
        cat $data
        """
}

workflow {
    Channel.fromList(['1','2','3','4']) | fitData | view
}
