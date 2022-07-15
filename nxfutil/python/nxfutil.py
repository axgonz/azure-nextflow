import os
import requests
import re
import subprocess
from azure.keyvault.secrets import SecretClient
from azure.identity import DefaultAzureCredential

def curl(uri, fileName=""):
    if fileName == "":
        fileName = uri.split("/")[-1]

    response = requests.get(uri)
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

def replaceParams(Path, text, subs, flags=0):
  with open(filepath, "r+") as f1:
       contents = f1.read()
       pattern = re.compile(re.escape(text), flags)
       contents = pattern.sub(subs, contents)
       f1.seek(0)
       f1.truncate()
       f1.write(file_contents)

curl("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/nextflow.config")
curl("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/pipeline.nf")
curl("https://raw.githubusercontent.com/axgonz/azure-nextflow/main/nextflow/pipelines/helloWorld/parameters.json")

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

