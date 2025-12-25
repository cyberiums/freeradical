#!/bin/bash
set -e

# Ensure openapi-generator-cli is installed (via npx)
GENERATOR="npx @openapitools/openapi-generator-cli"

echo "🚀 Generating FreeRadical SDKs..."

# Python SDK
echo "🐍 Generating Python SDK..."
$GENERATOR generate \
    -i docs/openapi.yaml \
    -g python \
    -o sdks/python \
    --additional-properties=packageName=freeradical_client,projectName=freeradical-client

# Go SDK
echo "🐹 Generating Go SDK..."
$GENERATOR generate \
    -i docs/openapi.yaml \
    -g go \
    -o sdks/go \
    --additional-properties=packageName=freeradical,projectName=freeradical-client-go

echo "✅ SDK Generation Complete!"
