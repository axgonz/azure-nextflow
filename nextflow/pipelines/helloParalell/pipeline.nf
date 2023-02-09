#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process listFiles {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val data

    output:
        stdout

    script:
        """
        # This process will fork to the max number of CPUs 
        #  available. If there are more inputs than available 
        #  CPUs the additional ones will be queued until a
        #  CPU is freed by a finishing process.
        
        echo process $data is sleeping
        sleep 30s
        echo process $data is done
        """
}

workflow {
    Channel.fromList(params.tasks) | listFiles | view
}
