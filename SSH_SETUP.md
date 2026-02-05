# SSH Setup Commands for Windows

## ✅ You've successfully created your SSH key!

Your new SSH key is at:
- Private: `C:\Users\Bruce.Mbudi\.ssh\id_ed25519`
- Public: `C:\Users\Bruce.Mbudi\.ssh\id_ed25519.pub`

---

## 🔧 Next Steps

### Step 1: Enable SSH Agent (Run PowerShell as Administrator)

**Right-click PowerShell → "Run as Administrator"**, then run:

```powershell
# Set ssh-agent to start automatically
Set-Service ssh-agent -StartupType Automatic

# Start the service
Start-Service ssh-agent

# Verify it's running
Get-Service ssh-agent
```

### Step 2: Add Your SSH Key (Regular PowerShell)

After the service is running, in your regular PowerShell:

```powershell
ssh-add $env:USERPROFILE\.ssh\id_ed25519
```

### Step 3: Copy Your Public Key to Clipboard

```powershell
Get-Content $env:USERPROFILE\.ssh\id_ed25519.pub | Set-Clipboard
```

### Step 4: Add SSH Key to GitHub

1. Go to: **https://github.com/settings/keys**
2. Click **"New SSH key"**
3. Title: `Windows Desktop - Kaizen Flow`
4. Paste your key (Ctrl+V)
5. Click **"Add SSH key"**

### Step 5: Test SSH Connection

```bash
ssh -T git@github.com
```

Expected output:
```
Hi bruceowenga! You've successfully authenticated, but GitHub does not provide shell access.
```

### Step 6: Push to GitHub

```bash
cd c:\Users\Bruce.Mbudi\prod_projects\kaizen-flow
git push -u origin main
```

---

## 🚨 If You Don't Have Admin Access

Use HTTPS instead of SSH:

```bash
# Change remote to HTTPS
git remote set-url origin https://github.com/bruceowenga/kaizen-flow.git

# Push (will prompt for username and Personal Access Token)
git push -u origin main
```

To create a Personal Access Token:
1. Go to: https://github.com/settings/tokens
2. Click "Generate new token (classic)"
3. Select scopes: `repo` (all)
4. Copy the token and use it as your password when pushing

---

## 📋 Quick Reference

**Your public key:**
```
C:\Users\Bruce.Mbudi\.ssh\id_ed25519.pub
```

**GitHub SSH settings:**
https://github.com/settings/keys

**GitHub token settings:**
https://github.com/settings/tokens
