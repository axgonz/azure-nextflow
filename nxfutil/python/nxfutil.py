import os
import re
import subprocess

import requests
import argparse

from azure.keyvault.secrets import SecretClient
from azure.identity import DefaultAzureCredential

parser = argparse.ArgumentParser()
parser.add_argument("-c",
    "--config-uri", 
    help="Uri to nextflow config ('.config') file.",
    type=str,
    default="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config")
parser.add_argument("-p",
    "--pipeline-uri", 
    help="Uri to nextflow pipeline ('.nf') file.",
    type=str,
    default="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf")    
parser.add_argument("-a",
    "--parameters-uri", 
    help="Uri to nextflow parameters ('.json') file.",
    type=str,
    default="https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json")
parser.add_argument("--version", action="version", version='%(prog)s - Version 1.0')
args = parser.parse_args()

def curl(uri, fileName=""):
    h = {
        "Cache-Control": "no-cache",
        "Pragma": "no-cache"
    }

    if fileName == "":
        fileName = uri.split("/")[-1]

    response = requests.get(uri, headers=h)
    if response.status_code == 200:
        f = open(fileName, "w")
        f.write(response.text)
        f.close()
        print(fileName)
    else:
        print(f"Error {response.status_code} downloading: {uri}")

def findFirstInLine(fileName, pattern):
    list = []    
    f = open(fileName, "r")
    for line in f:
        match = re.findall(pattern, line)
        if match:
            list.append(match[0])
    return list

def findSecrets(fileName):
    list = []
    matches = findFirstInLine(fileName, "secrets.[a-z,A-Z,_]*")
    for item in matches:
        item = item.split(".")[1]
        if item not in list:
            list.append(item)
    return list

def findParams(fileName):
    list = []
    matches = findFirstInLine(fileName, "exParams.[a-z,A-Z,_]*")
    for item in matches:
        item = item.split(".")[1]
        if item not in list:
            list.append(item)
    return list

def replaceParams(fileName, text, subs, flags=0):
  with open(fileName, "r+") as f1:
       contents = f1.read()
       pattern = re.compile(re.escape(text), flags)
       contents = pattern.sub(subs, contents)
       f1.seek(0)
       f1.truncate()
       f1.write(contents)

curl(args.config_uri, "nextflow.config")
curl(args.pipeline_uri, "pipeline.nf")
curl(args.parameters_uri, "parameters.json")

secrets = findSecrets("nextflow.config")
params = findParams("nextflow.config")

keyVaultName = os.environ["AZ_KEY_VAULT_NAME"]
KVUri = f"https://{keyVaultName}.vault.azure.net"

credential = DefaultAzureCredential()
client = SecretClient(vault_url=KVUri, credential=credential)

for secret in secrets:
    print(f"Importing secret '{secret.replace('_','-')}' to nextflow as '{secret}'")
    azSecret = client.get_secret(secret.replace("_","-"))
    subprocess.run(["./nextflow", "secrets", "put", "-n", secret, "-v", azSecret.value])

for param in params:
    print(f"Extending param '{secret.replace('_','-')}'")
    azSecret = client.get_secret(param.replace("_","-"))
    replaceParams("nextflow.config", f"exParams.{param}", azSecret.value)

subprocess.run(["./nextflow", "config"])
subprocess.run(["./nextflow", "run", "pipeline.nf", "-params-file", "parameters.json", "-w", "az://batch/work", "-with-timeline", "-with-dag"])

