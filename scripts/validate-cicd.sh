#!/bin/bash

# Validate GitHub Actions Workflow
# This script checks if the publish-sdks.yml workflow is valid

set -e

WORKFLOW_FILE=".github/workflows/publish-sdks.yml"

echo "🔍 Validating GitHub Actions workflow..."

# Check if file exists
if [ ! -f "$WORKFLOW_FILE" ]; then
    echo "❌ Workflow file not found: $WORKFLOW_FILE"
    exit 1
fi

echo "✅ Workflow file exists"

# Check required secrets are documented
echo ""
echo "📋 Required GitHub Secrets:"
echo "  - NPM_TOKEN (for TypeScript SDK)"
echo "  - PYPI_TOKEN (for Python SDK)"
echo ""

# Check if workflow has correct triggers
if grep -q "typescript-v\*" "$WORKFLOW_FILE" && \
   grep -q "python-v\*" "$WORKFLOW_FILE" && \
   grep -q "go-v\*" "$WORKFLOW_FILE"; then
    echo "✅ Tag triggers configured correctly"
else
    echo "❌ Tag triggers missing or incorrect"
    exit 1
fi

# Check if jobs exist
if grep -q "publish-typescript:" "$WORKFLOW_FILE" && \
   grep -q "publish-python:" "$WORKFLOW_FILE" && \
   grep -q "publish-go:" "$WORKFLOW_FILE"; then
    echo "✅ All three publish jobs defined"
else
    echo "❌ Missing publish jobs"
    exit 1
fi

echo ""
echo "✅ Workflow validation passed!"
echo ""
echo "📝 Next steps:"
echo "1. Add NPM_TOKEN secret to GitHub repository"
echo "2. Add PYPI_TOKEN secret to GitHub repository"
echo "3. Push a tag to trigger publishing:"
echo "   - TypeScript: git tag typescript-v0.7.0 && git push origin typescript-v0.7.0"
echo "   - Python:     git tag python-v1.0.0 && git push origin python-v1.0.0"
echo "   - Go:         git tag go-v1.0.0 && git push origin go-v1.0.0"
echo ""
echo "📊 Monitor at: https://github.com/cyberiums/freeradical/actions"
