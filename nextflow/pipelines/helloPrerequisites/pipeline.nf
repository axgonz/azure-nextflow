#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process prerequisites {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    output:
        stdout

    script:
        """
        echo 'Hello, I am the contents of the prerequisite file.' > $params.azureFileShare/file.txt
        sleep 10s
        echo $params.azureFileShare/file.txt
        """
}

process parallel {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val fileName
        val i

    output:
        stdout

    script:
        """
        echo -n "Parallel task #$i reading prerequisite file: " 
        cat $fileName 
        """
}

workflow {
    def fileName = prerequisites()
    parallel(fileName, Channel.fromList(["1","2","3","4","5","6"])) | view
}

