#!/bin/bash
set -e
workspace=$(cd "$(dirname "$0")" && pwd -P)
{
    cd "$workspace"
    # Check if terraform is installed
    if ! command -v terraform &>/dev/null; then
        echo "terraform not found. Installing..."
        brew install terraform
    else
        echo "terraform already installed"
    fi

    # start the keycloak
    sh ../services-up.sh

    # initialize the terraform, only at first time
    terraform init

    # load the google env from ../../.env
    source load-google-env.sh

    terraform plan

    # apply the keycloak configuration
    terraform apply -auto-approve
}
