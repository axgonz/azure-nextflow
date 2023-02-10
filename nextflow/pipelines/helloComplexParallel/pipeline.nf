#!/usr/bin/env nextflow

nextflow.enable.dsl=2

process parallel {
    cpus "$params.cpusPerSample"
    queue 'default'
    container "$params.azureRegistryServer/default/cipa:latest"

    input:
        val sample

    output:
        stdout

    script:
        """
        Rscript CiPA/hERG_fitting/generate_bootstrap_samples.R -d $params.drugName > "ref_${params.drugName}_${sample}.txt"
        cp "ref_${params.drugName}_${sample}.txt" "${params.azureFileShare}/ref_${params.drugName}_${sample}.txt"

        Rscript CiPA/hERG_fitting/hERG_fitting.R -d $params.drugName -c $task.cpus -i $sample -l $params.population -t $params.accuracy > "fit_${params.drugName}_${sa>        cp "fit_${params.drugName}_${sample}.txt" "${params.azureFileShare}/fit_${params.drugName}_${sample}.txt"
        """
}

workflow {
    Channel.from(0..(params.numberOfSamples-1)) | parallel | view
}
