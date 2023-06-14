#!/bin/bash
set -e
workspace=$(cd "$(dirname "$0")" && pwd -P)
cd "$workspace"
# https://cloud.google.com/sdk/docs/install
if [ ! -f google-cloud-sdk-435.0.0-darwin-arm.tar.gz ]; then
    curl -O https://dl.google.com/dl/cloudsdk/channels/rapid/downloads/google-cloud-sdk-435.0.0-darwin-arm.tar.gz
    tar xf google-cloud-sdk-435.0.0-darwin-arm.tar.gz
    cd google-cloud-sdk || exit
    bin/gcloud init
fi
alias gcloud="google-cloud-sdk/bin/gcloud"
gcloud auth application-default login

# initialize the terraform, only at first time
terraform init

terraform plan

# apply the keycloak configuration
terraform apply -auto-approve
