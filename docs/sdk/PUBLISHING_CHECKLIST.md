# SDK Publishing Checklist

Use this checklist when publishing any SDK version.

---

## Pre-Release

### All SDKs
- [ ] **Version Sync**: Ensure all SDKs use same version number
- [ ] **Tests Pass**: Run all test suites
- [ ] **Documentation**: Update READMEs with new features
- [ ] **Changelog**: Update CHANGELOG.md with release notes
- [ ] **Dependencies**: Review and update dependencies

### TypeScript SDK
- [ ] `npm install` successful
- [ ] `npm run build` successful
- [ ] `npm test` passes
- [ ] `package.json` version updated
- [ ] TypeScript compilation has no errors

### Python SDK
- [ ] `setup.py` version updated
- [ ] `pyproject.toml` version updated
- [ ] `pytest` passes
- [ ] Requirements validated

### Go SDK
- [ ] `go test ./...` passes
- [ ] `go mod tidy` run
- [ ] No deprecated dependencies

---

## Publishing

### TypeScript → npm
```bash
cd sdk/freeradical-sdk
npm version 0.7.1
npm publish --access public
```

### Python → PyPI
```bash
cd sdks/python
python setup.py sdist bdist_wheel
twine upload dist/*
```

### Go → GitHub
```bash
git tag v0.7.1
git push origin v0.7.1
```

### Automated (Recommended)
```bash
./scripts/publish-sdk.sh typescript 0.7.1
./scripts/publish-sdk.sh python 0.7.1
./scripts/publish-sdk.sh go 0.7.1
```

---

## Post-Release

### Verification
- [ ] **npm**: `npm info @freeradical/sdk`
- [ ] **PyPI**: `pip install freeradical-client==0.7.1`
- [ ] **Go**: `go get github.com/cyberiums/freeradical-go-client@v0.7.1`

### Test Installation
```bash
# TypeScript
mkdir test-ts && cd test-ts
npm init -y
npm install @freeradical/sdk
node -e "const client = require('@freeradical/sdk'); console.log('OK')"

# Python
python -m venv test-py && source test-py/bin/activate
pip install freeradical-client
python -c "import freeradical_client; print('OK')"

# Go
mkdir test-go && cd test-go
go mod init test
go get github.com/cyberiums/freeradical-go-client@v0.7.1
```

### Communication
- [ ] Create GitHub release with notes
- [ ] Update documentation site
- [ ] Announce on:
  - [ ] Twitter/X
  - [ ] Dev.to
  - [ ] Reddit r/rust, r/webdev
  - [ ] Discord/Slack communities

### Monitoring
- [ ] Monitor npm downloads
- [ ] Monitor PyPI downloads
- [ ] Watch for GitHub issues
- [ ] Check error tracking (if integrated)

---

## Rollback Procedure

If a release has critical bugs:

### NPM
```bash
npm unpublish @freeradical/sdk@0.7.1
# Note: Only works within 72 hours
```

### PyPI
Cannot unpublish, but can:
```bash
# Upload hotfix version
python setup.py sdist bdist_wheel
twine upload dist/*
# Then mark version as "yanked" in PyPI web UI
```

### Go
```bash
# Delete tag
git tag -d v0.7.1
git push origin :refs/tags/v0.7.1
# Users who already downloaded will keep it
```

---

## Version Matrix

Keep this updated after each release:

| Version | TypeScript | Python | Go | Date |
|---------|-----------|--------|-----|------|
| 0.7.0   | ✅ | ✅ | ✅ | 2025-12-24 |
| 0.7.1   | ⏳ | ⏳ | ⏳ | Pending |

---

## GitHub Actions

Automated publishing via tags:

```bash
# Trigger TypeScript publish
git tag typescript-v0.7.1 && git push origin typescript-v0.7.1

# Trigger Python publish
git tag python-v0.7.1 && git push origin python-v0.7.1

# Trigger Go publish
git tag go-v0.7.1 && git push origin go-v0.7.1
```

Requires secrets:
- `NPM_TOKEN`
- `PYPI_TOKEN`

---

**Last Updated**: 2025-12-24
