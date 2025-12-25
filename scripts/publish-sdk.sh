#!/bin/bash

# SDK Publishing Script
# Usage: ./scripts/publish-sdk.sh [typescript|python|go] [version]

set -e

SDK_TYPE=$1
VERSION=$2

if [ -z "$SDK_TYPE" ] || [ -z "$VERSION" ]; then
    echo "Usage: ./scripts/publish-sdk.sh [typescript|python|go] [version]"
    echo "Example: ./scripts/publish-sdk.sh typescript 0.7.1"
    exit 1
fi

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}📦 Publishing $SDK_TYPE SDK v$VERSION${NC}"

case $SDK_TYPE in
    typescript)
        echo -e "${YELLOW}Building TypeScript SDK...${NC}"
        cd sdk/freeradical-sdk
        
        # Update version
        npm version $VERSION --no-git-tag-version
        
        # Build
        npm install
        npm run build
        
        # Publish
        echo -e "${YELLOW}Publishing to npm...${NC}"
        npm publish --access public
        
        # Tag
        TAG="typescript-v$VERSION"
        echo -e "${YELLOW}Creating git tag: $TAG${NC}"
        cd ../..
        git add sdk/freeradical-sdk/package.json
        git commit -m "chore: release typescript sdk v$VERSION"
        git tag $TAG
        git push origin $TAG
        
        echo -e "${GREEN}✅ TypeScript SDK published!${NC}"
        echo -e "Install with: ${YELLOW}npm install @freeradical/sdk@$VERSION${NC}"
        ;;
        
    python)
        echo -e "${YELLOW}Building Python SDK...${NC}"
        cd sdks/python
        
        # Update version in setup.py
        sed -i.bak "s/VERSION = \".*\"/VERSION = \"$VERSION\"/" setup.py
        rm setup.py.bak
        
        # Clean previous builds
        rm -rf dist/ build/ *.egg-info
        
        # Build
        python setup.py sdist bdist_wheel
        
        # Check
        twine check dist/*
        
        # Publish
        echo -e "${YELLOW}Publishing to PyPI...${NC}"
        twine upload dist/*
        
        # Tag
        TAG="python-v$VERSION"
        echo -e "${YELLOW}Creating git tag: $TAG${NC}"
        cd ../..
        git add sdks/python/setup.py
        git commit -m "chore: release python sdk v$VERSION"
        git tag $TAG
        git push origin $TAG
        
        echo -e "${GREEN}✅ Python SDK published!${NC}"
        echo -e "Install with: ${YELLOW}pip install freeradical-client==$VERSION${NC}"
        ;;
        
    go)
        echo -e "${YELLOW}Publishing Go SDK...${NC}"
        cd sdks/go
        
        # Run tests
        go test ./...
        
        # Tag (Go uses semantic version tags)
        TAG="v$VERSION"
        echo -e "${YELLOW}Creating git tag: $TAG${NC}"
        cd ../..
        git commit --allow-empty -m "chore: release go sdk v$VERSION"
        git tag $TAG
        git push origin $TAG
        
        echo -e "${GREEN}✅ Go SDK published!${NC}"
        echo -e "Install with: ${YELLOW}go get github.com/cyberiums/freeradical-go-client@$TAG${NC}"
        ;;
        
    *)
        echo -e "${RED}❌ Unknown SDK type: $SDK_TYPE${NC}"
        echo "Valid types: typescript, python, go"
        exit 1
        ;;
esac

echo -e "${GREEN}🎉 SDK v$VERSION published successfully!${NC}"
