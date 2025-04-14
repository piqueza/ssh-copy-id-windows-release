# 🔐 ssh-copy-id for Windows (Rust Edition)

A fast, native Windows CLI utility to install your SSH public key on a remote Linux/Unix server — just like the classic `ssh-copy-id` on Linux!

✅ Written in Rust  
✅ Designed for Windows 10/11  
✅ Installs cleanly into `Program Files`  
✅ Supports elevation, user-friendly CLI, and automatic PATH setup

---

## 🧩 Features

- 📂 Appends your public key to `~/.ssh/authorized_keys` on a remote host
- 🔐 Uses SSH Agent or password auth
- ✅ No external dependencies — runs natively on Windows
- ⚡ Installs with `--install`, removes with `--uninstall`
- 🧠 Adds itself to PATH for global terminal access
- 🧼 Automatically cleans up temporary install scripts

---

## 📥 Installation

### 📦 Option 1: Install via Command Line (First-Time Setup)

```CMD or Powershell:
ssh-copy-id.exe --install
