# GitHub Secrets Configuration

To enable automated SDK publishing, you need to add two secrets to your GitHub repository.

## 📍 Repository Secrets URL

**Go to**: https://github.com/cyberiums/freeradical/settings/secrets/actions

---

## 1️⃣ NPM_TOKEN

**For**: TypeScript/JavaScript SDK publishing to npm

### Generate Token

```bash
# Login to npm
npm login

# Create access token
npm token create
```

Copy the token (starts with `npm_...`)

### Add to GitHub

1. Go to: https://github.com/cyberiums/freeradical/settings/secrets/actions
2. Click **"New repository secret"**
3. **Name**: `NPM_TOKEN`
4. **Value**: Paste your npm token
5. Click **"Add secret"**

---

## 2️⃣ PYPI_TOKEN

**For**: Python SDK publishing to PyPI

### Generate Token

1. Visit: https://pypi.org/manage/account/token/
2. Click **"Add API token"**
3. **Token name**: `freeradical-sdk-upload`
4. **Scope**: "Entire account" or specific to `freeradical-client`
5. Copy the token (starts with `pypi-...`)

### Add to GitHub

1. Go to: https://github.com/cyberiums/freeradical/settings/secrets/actions
2. Click **"New repository secret"**
3. **Name**: `PYPI_TOKEN`
4. **Value**: Paste your PyPI token
5. Click **"Add secret"**

---

## ✅ Verification

Once both secrets are added:

1. Go to: https://github.com/cyberiums/freeradical/settings/secrets/actions
2. You should see:
   - `NPM_TOKEN`
   - `PYPI_TOKEN`
   - `GITHUB_TOKEN` (automatically provided)

---

## 🚀 Test the Setup

Push a test tag to verify:

```bash
git tag python-v1.0.0
git push origin python-v1.0.0
```

Monitor the workflow:
https://github.com/cyberiums/freeradical/actions

If successful, you'll see:
- ✅ Workflow completed
- 📦 Package published to PyPI
- 📝 GitHub release created

---

**Note**: Go SDK doesn't require a secret (published via git tags only)
