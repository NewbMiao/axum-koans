#!/bin/bash

# Load variables from .env file
set -a
source ../../.env
set +a

# Check if variables are empty
if [ -z "$GOOGLE_CLIENT_ID" ] || [ -z "$GOOGLE_CLIENT_SECRET" ]; then
    echo "Error: GOOGLE_CLIENT_ID or GOOGLE_CLIENT_SECRET is not set in .env file."
    exit 1
fi
# Export variables
export TF_VAR_google_client_id="$GOOGLE_CLIENT_ID"
export TF_VAR_google_client_secret="$GOOGLE_CLIENT_SECRET"
