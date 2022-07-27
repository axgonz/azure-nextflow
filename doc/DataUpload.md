# Data upload

## Upload

- Files are tar'd and hash/checksum on archive is taken.
- Checksum becomes unique id.
- Tar is uploaded to Storage File as target.
    - Strip File metadata.
    - Tag with File metadata "unscanned".
    - Tag with unique id for sample correlation.
- Message on Storage Queue is generated using unique id.

## File validation

- A Docker image with ClamAV installed.
    - CMD to untar archive.
    - CMD to download latest virus definitions.
    - CMD to start an [on-demand](https://linuxconfig.org/scan-ubuntu-18-04-for-viruses-with-clamav) scan.
    - CMD to do any other file validation steps needed.
    - CMD to tag as "scanned-ok" or "scanned-error".
    - CMD to call nextflow Functaion App, or, message on Storage Queue.
- Container Instance to run image.
- Container Instance has file share mounted at creation time.

## Trigger

- Upload Function App to watch for Storage Queue message.
- Upload Function App to create Container Instance for file validation.
- Function App passes unique id to Container Instnace so CMD can locate tar archive.