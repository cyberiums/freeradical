# FreeRadical CMS v1.0.1 - Publication Checklist

**Target Release Date**: December 25, 2025  
**Version**: 1.0.1 (unified across all packages)

---

## ✅ Pre-Release Verification

### Code Quality
- [x] All compilation errors resolved
- [x] No critical warnings
- [x] Commerce API functional (Products + Orders)
- [x] Payment integration tested
- [ ] Run full test suite: `cargo test`
- [ ] Run integration tests

### Version Numbers
- [x] Main project: `Cargo.toml` → v1.0.1
- [x] TypeScript SDK: `package.json` → v1.0.1
- [x] Python SDK: `setup.py` → v1.0.1
- [ ] Go SDK: Verify tag `go-v1.0.1`
- [ ] Python SDK: `pyproject.toml` → v1.0.1

### Documentation
- [x] CHANGELOG.md created
- [x] Release notes created (`docs/releases/RELEASE-NOTES-v1.0.1.md`)
- [x] README.md updated
- [x] SDK Developer Guide complete
- [x] Core Developer Guide complete
- [x] API documentation current
- [ ] Update main README with v1.0.1 badge

---

## 🛠️ Build Verification

### Main Application
```bash
# Clean build
cargo clean
cargo build --release

# Verify binary
./target/release/freeradical --version
# Expected: freeradical 1.0.1
```

### TypeScript SDK
```bash
cd sdk/freeradical-sdk
npm install
npm run build
npm pack  # Verify package

# Expected output: freeradical-sdk-1.0.1.tgz
```

### Python SDK
```bash
cd sdks/python
rm -rf dist/ build/ *.egg-info
python setup.py sdist bdist_wheel
twine check dist/*

# Expected: PASSED
```

### Go SDK
```bash
cd sdks/go
go mod tidy
go test ./...
go build .

# Expected: clean build
```

---

## 📦 GitHub Preparation

### Repository Tasks
- [ ] Commit all changes
  ```bash
  git add .
  git commit -m "Release v1.0.1: Commerce API & SDK Ecosystem"
  ```

- [ ] Create and push tags
  ```bash
  # Main release
  git tag -a v1.0.1 -m "v1.0.1: Commerce API & SDK Ecosystem"
  git push origin v1.0.1
  
  # SDK tags (triggers CI/CD)
  git tag typescript-v1.0.1
  git tag python-v1.0.1
  git tag go-v1.0.1
  
  git push origin typescript-v1.0.1 python-v1.0.1 go-v1.0.1
  ```

### GitHub Secrets
- [ ] Verify `NPM_TOKEN` is configured
- [ ] Verify `PYPI_TOKEN` is configured
- [ ] Test automated publishing workflow

---

## 🚀 SDK Publication

### TypeScript SDK (npm)

**Automated** (via GitHub Actions when tag pushed):
```bash
git push origin typescript-v1.0.1
```

**Manual** (if needed):
```bash
cd sdk/freeradical-sdk
npm publish --access public
```

**Verification**:
- [ ] Package appears on npm: https://www.npmjs.com/package/@freeradical/sdk
- [ ] Version 1.0.1 listed
- [ ] Installation works: `npm install @freeradical/sdk@1.0.1`

### Python SDK (PyPI)

**Automated** (via GitHub Actions when tag pushed):
```bash
git push origin python-v1.0.1
```

**Manual** (if needed):
```bash
cd sdks/python
twine upload dist/*
```

**Verification**:
- [ ] Package appears on PyPI: https://pypi.org/project/freeradical-client/
- [ ] Version 1.0.1 listed
- [ ] Installation works: `pip install freeradical-client==1.0.1`

### Go SDK (GitHub)

**Automated** (via GitHub tag):
```bash
git push origin go-v1.0.1
```

**Verification**:
- [ ] Tag appears on GitHub releases
- [ ] Installation works: `go get github.com/cyberiums/freeradical-go-client@go-v1.0.1`
- [ ] Package appears on pkg.go.dev

---

## 📝 GitHub Release

### Create Release on GitHub

1. Go to: https://github.com/cyberiums/freeradical/releases/new
2. Select tag: `v1.0.1`
3. Release title: `v1.0.1 - Commerce API & SDK Ecosystem`
4. Description: Copy from `docs/releases/RELEASE-NOTES-v1.0.1.md`
5. Attachments:
   - [ ] Source code (auto-generated)
   - [ ] Build artifacts (optional)
6. Click **"Publish release"**

---

## ✅ Post-Release Verification

### Installation Tests

**TypeScript**:
```bash
mkdir test-ts && cd test-ts
npm init -y
npm install @freeradical/sdk@1.0.1
node -e "const sdk = require('@freeradical/sdk'); console.log('✅ OK')"
```

**Python**:
```bash
python -m venv test-py && source test-py/bin/activate
pip install freeradical-client==1.0.1
python -c "import freeradical_client; print('✅ OK')"
```

**Go**:
```bash
mkdir test-go && cd test-go
go mod init test
go get github.com/cyberiums/freeradical-go-client@go-v1.0.1
go run -e "package main; import _ \"github.com/cyberiums/freeradical-go-client\"; func main() {}"
```

### GitHub Actions
- [ ] Check workflow status: https://github.com/cyberiums/freeradical/actions
- [ ] Verify all SDK publish jobs succeeded
- [ ] Review logs for any warnings

### Monitoring
- [ ] npm downloads tracking
- [ ] PyPI downloads tracking
- [ ] GitHub stars/forks
- [ ] Issue tracker clean

---

## 📢 Announcement

### Update Documentation
- [ ] Update main README.md with v1.0.1 features
- [ ] Update docs/README.md
- [ ] Add v1.0.1 to version history

### Social Media (Optional)
- [ ] Twitter/X announcement
- [ ] LinkedIn post
- [ ] Reddit r/rust, r/webdev
- [ ] Hacker News (Show HN)
- [ ] Dev.to article

### Community
- [ ] Post in GitHub Discussions
- [ ] Update project website (if applicable)
- [ ] Email newsletter (if applicable)

---

## 🐛 Rollback Plan

If critical issues are discovered:

### Unpublish from npm
```bash
npm unpublish @freeradical/sdk@1.0.1
```

### Unpublish from PyPI
PyPI doesn't allow unpublishing. Instead:
1. Create hotfix version 1.0.2
2. Document the issue

### Remove GitHub Tag
```bash
git tag -d v1.0.1
git push origin :refs/tags/v1.0.1
```

### Create Hotfix
1. Create branch: `hotfix/v1.0.2`
2. Fix issue
3. Follow this checklist again for v1.0.2

---

## 📋 Success Criteria

- [x] All version numbers at 1.0.1
- [x] All documentation complete
- [x] All SDKs built and validated
- [ ] All tests passing
- [ ] All SDKs published successfully
- [ ] GitHub release created
- [ ] Installation verified on all platforms
- [ ] No critical issues reported within 24 hours

---

## 🎉 Release Complete!

Once all checkboxes are ticked, the v1.0.1 release is complete!

**Next Steps**:
1. Monitor for issues
2. Respond to community feedback
3. Plan v1.1.0 features
4. Continue development

---

**Release Manager**: FreeRadical Team  
**Date**: December 25, 2025  
**Status**: 🚧 In Progress
