🔐 ssh-copy-id for Windows is a native, Rust-powered CLI utility that securely copies your public SSH key to a remote Linux/Unix server — just like the built-in tool on Linux.

✅ Built specifically for Windows
✅ No admin rights required
✅ Works from CMD, PowerShell, or Windows Terminal
✅ Adds itself to your user PATH
✅ Fast, safe, portable

🚀 Features

- 📂 Appends your public key to ~/.ssh/authorized_keys
- 🔐 Supports password auth and SSH agent
- 🧰 Installs to %LOCALAPPDATA%\Programs\ssh-copy-id
- 🛠️ CLI flags: --install, --uninstall, --key, --show-password
- 🤖 Written in Rust for performance and reliability

📦 How to Install
1. Download: ssh-copy-id-windows.zip
2. Extract anywhere
3. Run in terminal:
`.\ssh-copy-id.exe --install`
4. ✅ The tool will copy itself to:
`C:\Users\<you>\AppData\Local\Programs\ssh-copy-id\`
and update your PATH automatically.
5. Open a new terminal, and run:
`ssh-copy-id --help`

🧪 Example Usage
`ssh-copy-id --user root --host your.server.com`
`ssh-copy-id --user user --host host.com --key C:\Users\You\.ssh\id_ed25519.pub`

🧼 Uninstall
`ssh-copy-id --uninstall`
Removes the binary and cleans your PATH.

📄 License
MIT — use it, fork it, ship it.
