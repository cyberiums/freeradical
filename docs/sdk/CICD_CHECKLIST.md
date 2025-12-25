# CI/CD Configuration Checklist

Use this checklist to configure automated SDK publishing.

---

## ✅ Pre-Configuration

- [x] GitHub Actions workflow created (`.github/workflows/publish-sdks.yml`)
- [x] Validation script created (`scripts/validate-cicd.sh`)
- [x] All SDKs built and tested locally

---

## 🔐 GitHub Secrets Setup

### Required Secrets

- [ ] **NPM_TOKEN** - For TypeScript SDK publishing
  - Generate: `npm token create`
  - Add at: https://github.com/cyberiums/freeradical/settings/secrets/actions
  
- [ ] **PYPI_TOKEN** - For Python SDK publishing
  - Generate: https://pypi.org/manage/account/token/
  - Add at: https://github.com/cyberiums/freeradical/settings/secrets/actions

---

## 🧪 Testing Workflow

### 1. Validate Workflow Locally

```bash
./scripts/validate-cicd.sh
```

Expected output: ✅ All checks pass

### 2. Test with Python SDK (Recommended First)

```bash
# Ensure package builds
cd sdks/python
rm -rf dist/
python setup.py sdist bdist_wheel

# Create and push tag
git tag python-v1.0.0-test
git push origin python-v1.0.0-test

# Monitor workflow
# Visit: https://github.com/cyberiums/freeradical/actions
```

### 3. Verify Publication

- [ ] Workflow completed successfully
- [ ] Package appears on PyPI: https://pypi.org/project/freeradical-client/
- [ ] Can install: `pip install freeradical-client==1.0.0`

---

## 🚀 Production Publishing

Once testing succeeds, publish all SDKs:

### TypeScript SDK

```bash
git tag typescript-v0.7.0
git push origin typescript-v0.7.0
```

**Verify**: https://www.npmjs.com/package/@freeradical/sdk

### Python SDK

```bash
git tag python-v1.0.0
git push origin python-v1.0.0
```

**Verify**: https://pypi.org/project/freeradical-client/

### Go SDK

```bash
git tag go-v1.0.0
git push origin go-v1.0.0
```

**Verify**: https://pkg.go.dev/github.com/cyberiums/freeradical-go-client

---

## 📊 Post-Publishing

- [ ] All three workflows completed successfully
- [ ] All packages installable from registries
- [ ] GitHub releases created
- [ ] Update documentation with new versions
- [ ] Announce releases on social media

---

## 🔄 Maintenance

### Regular Updates

- Review and update workflow dependencies quarterly
- Rotate API tokens annually
- Test workflow after any GitHub Actions platform changes

### Version Updates

When bumping versions:
1. Update version in package files
2. Build and test locally
3. Commit changes
4. Create and push tag
5. Monitor automated publishing

---

## 🐛 Troubleshooting

### Workflow Failed

1. Check workflow logs: https://github.com/cyberiums/freeradical/actions
2. Verify secrets are configured
3. Test locally with `./scripts/publish-sdk.sh`
4. Re-run workflow if transient failure

### Package Not Appearing

- **npm**: Can take 5-10 minutes to appear in search
- **PyPI**: Usually instant, check project page
- **Go**: Use specific version: `go get ...@v1.0.0`

---

**Status**: 
- [x] CI/CD configured
- [ ] Secrets added
- [ ] First test publish
- [ ] Production publish

**Last Updated**: 2025-12-25
