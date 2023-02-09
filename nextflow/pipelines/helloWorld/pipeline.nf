#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process splitLetters {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    output:
        path 'chunk_*'
        path 'script.log'

    script:
        """
        printf '${params.str}' | split -b 6 - chunk_
        echo foo >> '${param.azureFileShare}'/batchsmb/bar.txt
        chmod +x '${param.azureFileShare}'/batchsmb/script.sh
        '${param.azureFileShare}'/batchsmb/script.sh
        """
}

process convertToUpper {
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        file x

    output:
        stdout

    script:
        """
        cat $x | tr '[a-z]' '[A-Z]'
        """
}

workflow {
    splitLetters | flatten | convertToUpper | view { it.trim() }
}

