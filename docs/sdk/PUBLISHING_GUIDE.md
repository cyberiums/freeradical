# Publishing FreeRadical CMS SDKs

This guide covers publishing all FreeRadical CMS client SDKs to their respective package registries.

---

## 📦 TypeScript/JavaScript SDK → npm

### Prerequisites

```bash
# Install dependencies
cd sdk/freeradical-sdk
npm install

# Login to npm
npm login
```

### Pre-publish Checklist

- [ ] Update version in `package.json`
- [ ] Update `CHANGELOG.md`
- [ ] Run tests: `npm test`
- [ ] Build: `npm run build`
- [ ] Check bundle: `npm pack` (creates tarball)

### Publishing

```bash
cd sdk/freeradical-sdk

# Bump version (major.minor.patch)
npm version patch  # or minor, major

# Build distribution
npm run build

# Publish to npm
npm publish --access public

# Tag in git
git tag v$(node -p "require('./package.json').version")
git push origin --tags
```

### Verify

```bash
npm info @freeradical/sdk
npm install @freeradical/sdk
```

---

## 🐍 Python SDK → PyPI

### Prerequisites

```bash
# Install build tools
pip install --upgrade pip setuptools wheel twine

# Create PyPI account at https://pypi.org/
# Create API token at https://pypi.org/manage/account/token/
```

### Pre-publish Checklist

- [ ] Update version in `setup.py`
- [ ] Update version in `pyproject.toml`
- [ ] Run tests: `pytest`
- [ ] Update `README.md`

### Publishing

```bash
cd sdks/python

# Clean previous builds
rm -rf dist/ build/ *.egg-info

# Build distribution packages
python setup.py sdist bdist_wheel

# Check package
twine check dist/*

# Upload to TestPyPI (optional, for testing)
twine upload --repository testpypi dist/*

# Upload to PyPI (production)
twine upload dist/*

# Tag in git
git tag python-v$(python setup.py --version)
git push origin --tags
```

### Verify

```bash
pip install freeradical-client
python -c "import freeradical_client; print(freeradical_client.__version__)"
```

---

## 🔷 Go SDK → GitHub

Go packages are distributed via Git tags rather than a centralized registry.

### Prerequisites

```bash
# Ensure Go module is initialized
cd sdks/go
go mod tidy
go test ./...
```

### Pre-publish Checklist

- [ ] Update module version in code
- [ ] Run tests: `go test ./...`
- [ ] Update `README.md`
- [ ] Verify `go.mod` is correct

### Publishing

```bash
cd sdks/go

# Run tests
go test ./...

# Commit changes
git add .
git commit -m "chore: prepare go sdk v1.0.0"

# Tag version (MUST start with 'v')
git tag v1.0.0

# Push to GitHub
git push origin main
git push origin v1.0.0
```

### Verify

```bash
# Users can now install with:
go get github.com/{your-org}/freeradical-go-client@v1.0.0

# Or in go.mod:
# require github.com/{your-org}/freeradical-go-client v1.0.0
```

---

## 🔄 Automated Publishing (CI/CD)

### GitHub Actions Workflow

Create `.github/workflows/publish-sdks.yml`:

```yaml
name: Publish SDKs

on:
  push:
    tags:
      - 'typescript-v*'
      - 'python-v*'
      - 'go-v*'

jobs:
  publish-typescript:
    if: startsWith(github.ref, 'refs/tags/typescript-v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      - run: cd sdk/freeradical-sdk && npm install
      - run: cd sdk/freeradical-sdk && npm run build
      - run: cd sdk/freeradical-sdk && npm publish --access public
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}

  publish-python:
    if: startsWith(github.ref, 'refs/tags/python-v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.9'
      - run: pip install build twine
      - run: cd sdks/python && python -m build
      - run: cd sdks/python && twine upload dist/*
        env:
          TWINE_USERNAME: __token__
          TWINE_PASSWORD: ${{ secrets.PYPI_TOKEN }}

  publish-go:
    if: startsWith(github.ref, 'refs/tags/go-v')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-go@v4
        with:
          go-version: '1.21'
      - run: cd sdks/go && go test ./...
      # Go SDK is published via git tags (no additional action needed)
```

### Secrets Configuration

Add to GitHub repository secrets:
- `NPM_TOKEN` - npm access token
- `PYPI_TOKEN` - PyPI API token

---

## 📋 Versioning Strategy

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking API changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes

### Version Sync

Keep all SDKs on the same major version when possible:
- TypeScript: `0.7.0`
- Python: `0.7.0`
- Go: `v0.7.0`

### Tag Naming

- TypeScript: `typescript-v0.7.0`
- Python: `python-v0.7.0`
- Go: `v0.7.0` (Go convention)

---

## 📝 Release Checklist

Use this checklist for each release:

### Pre-release
- [ ] All tests passing
- [ ] Documentation updated
- [ ] CHANGELOG updated
- [ ] Version bumped
- [ ] README examples verified

### Release
- [ ] Build packages
- [ ] Test installation locally
- [ ] Publish to registry
- [ ] Create Git tags
- [ ] Push to repository

### Post-release
- [ ] Verify installation from registry
- [ ] Update documentation site
- [ ] Announce on social media
- [ ] Create GitHub release notes

---

## 🔗 Package URLs

Once published, packages will be available at:

- **npm**: https://www.npmjs.com/package/@freeradical/sdk
- **PyPI**: https://pypi.org/project/freeradical-client/
- **Go**: https://pkg.go.dev/github.com/{your-org}/freeradical-go-client

---

## 🐛 Troubleshooting

### PyPI: "403 Forbidden" Error

This is typically an **authentication issue**, not a package name conflict.

#### Root Cause
The `twine` command cannot authenticate because:
1. No `.pypirc` file exists
2. The API token is invalid or missing
3. Wrong token is being used (PyPI vs TestPyPI)

#### Solution: Set Up `.pypirc`

Create `~/.pypirc` with the following content:

```ini
[distutils]
index-servers =
    pypi
    testpypi

[pypi]
username = __token__
password = YOUR_PYPI_API_TOKEN_HERE

[testpypi]
repository = https://test.pypi.org/legacy/
username = __token__
password = YOUR_TESTPYPI_API_TOKEN_HERE
```

**Steps to complete setup:**

1. **Get your TestPyPI API token:**
   - Visit: https://test.pypi.org/manage/account/token/
   - Click "Add API token"
   - Set token name (e.g., "freeradical-sdk-upload")
   - Set scope to "Entire account" or specific project
   - **Copy the token immediately** (starts with `pypi-...`)

2. **Update `.pypirc`:**
   ```bash
   # Edit the file
   nano ~/.pypirc
   
   # Replace YOUR_TESTPYPI_API_TOKEN_HERE with your actual token
   # Example: password = pypi-AgEIcH...your-token-here
   ```

3. **Set correct permissions:**
   ```bash
   chmod 600 ~/.pypirc
   ```

4. **Retry upload:**
   ```bash
   cd sdks/python
   twine upload --repository testpypi dist/*
   ```

#### Alternative: Use Environment Variables

Instead of `.pypirc`, you can use environment variables:

```bash
export TWINE_USERNAME=__token__
export TWINE_PASSWORD=pypi-your-token-here
twine upload --repository testpypi dist/*
```

#### Verification

After successful upload, verify at:
- TestPyPI: https://test.pypi.org/project/freeradical-client/

---

### npm: "Package already exists"

```bash
# Version already published, bump version
npm version patch
```

### PyPI: "File already exists"

```bash
# Clean and rebuild
rm -rf dist/
python setup.py sdist bdist_wheel
```

### Go: "Invalid version"

```bash
# Ensure tag starts with 'v'
git tag v1.0.0  # Correct
git tag 1.0.0   # Wrong
```

---

## 📞 Support

For publishing issues:
- **npm**: https://docs.npmjs.com/
- **PyPI**: https://packaging.python.org/
- **Go**: https://go.dev/doc/modules/publishing

---

**Ready to publish!** 🚀

See individual SDK READMEs for language-specific details.
