#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process parallel {
    cpus "$params.cpusPerSample"
    queue 'default'
    container "$params.azureRegistryServer/default/ubuntu:latest"

    input:
        val sample

    output:
        stdout

    script:
        """
        echo "Generate ref for $params.drugName"
        echo "Fitting sample ${sample} on ${task.cpus} cpus"
        echo "I am results of sample ${sample} for ${params.drugName} with population ${params.population} and accuracy ${params.accuracy}." > "results_${params.drugName}_${sample}.txt"
        cp "results_${params.drugName}_${sample}.txt" "${params.azureFileShare}/results_${params.drugName}_${sample}.txt"
        """
}

workflow {
    Channel.from(0..(params.numberOfSamples-1)) | parallel | view
}
