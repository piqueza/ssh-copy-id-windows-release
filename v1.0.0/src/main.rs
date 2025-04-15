use anyhow::{anyhow, Context, Result};
use clap::Parser;
use dirs::{data_local_dir, home_dir};
use rpassword;
use ssh2::Session;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{exit, Command};

#[derive(Parser, Debug)]
#[command(name = "ssh-copy-id")]
#[command(version = "1.0")]
#[command(author = "Quintin & Rusty GPT")]
#[command(about = "Copies your SSH public key to a remote server via SSH")]
struct Cli {
    #[arg(short, long)]
    user: Option<String>,

    #[arg(short, long)]
    host: Option<String>,

    #[arg(short, long)]
    key: Option<PathBuf>,

    #[arg(long)]
    show_password: bool,

    #[arg(long)]
    install: bool,

    #[arg(long)]
    uninstall: bool,
}

fn get_default_pubkey_path() -> Result<PathBuf> {
    let mut path = home_dir().context("Could not determine home directory")?;
    path.push(".ssh/id_rsa.pub");
    Ok(path)
}

fn read_pubkey(pubkey_path: &PathBuf) -> Result<String> {
    if !pubkey_path.exists() {
        return Err(anyhow!(
            "❌ Public key file not found: {:?}\n\
            💡 Please generate one with `ssh-keygen`, or specify it with `--key <path>`",
            pubkey_path
        ));
    }

    Ok(fs::read_to_string(pubkey_path)?.trim().to_string())
}

fn ssh_copy_id(cli: &Cli, pubkey: &str) -> Result<()> {
    let user = cli.user.as_ref().context("Missing --user")?;
    let host = cli.host.as_ref().context("Missing --host")?;

    let tcp = TcpStream::connect(format!("{host}:22"))
        .with_context(|| format!("Could not connect to {host}:22"))?;

    let mut sess = Session::new().context("Failed to create SSH session")?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;

    if let Err(_) = sess.userauth_agent(user) {
        let password = if cli.show_password {
            print!("Enter password for {}@{}: ", user, host);
            io::stdout().flush().unwrap();
            let mut pw = String::new();
            io::stdin().read_line(&mut pw).unwrap();
            pw.trim().to_string()
        } else {
            rpassword::prompt_password(format!("Enter password for {}@{}: ", user, host))?
        };

        sess.userauth_password(user, &password)
            .context("Password authentication failed")?;
    }

    if !sess.authenticated() {
        return Err(anyhow::anyhow!("SSH authentication failed"));
    }

    let mut channel = sess.channel_session()?;
    channel.exec("mkdir -p ~/.ssh && chmod 700 ~/.ssh && touch ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys")?;
    let mut s = String::new();
    channel.read_to_string(&mut s).ok();
    channel.send_eof()?;
    channel.wait_eof()?;
    channel.wait_close()?;

    let sftp = sess.sftp()?;
    let path = PathBuf::from(".ssh/authorized_keys");

    let mut existing = String::new();
    if let Ok(mut file) = sftp.open(&path) {
        file.read_to_string(&mut existing)?;
    }

    if existing.contains(pubkey) {
        println!("🔐 Key already exists on remote server. Skipping.");
    } else {
        let mut file = sftp.open_mode(
            &path,
            ssh2::OpenFlags::WRITE | ssh2::OpenFlags::APPEND,
            0o600,
            ssh2::OpenType::File,
        )?;
        file.write_all(format!("\n{}\n", pubkey).as_bytes())?;
        println!("✅ Key added to remote authorized_keys.");
    }

    Ok(())
}

fn install_binary() -> Result<()> {
    let exe_path = env::current_exe()?;
    let install_dir = data_local_dir()
        .context("Unable to resolve AppData\\Local\\Programs")?
        .join("ssh-copy-id");
    let install_exe = install_dir.join("ssh-copy-id.exe");

    if !install_dir.exists() {
        fs::create_dir_all(&install_dir)?;
    }

    fs::copy(&exe_path, &install_exe)
        .with_context(|| format!("Failed to copy to {}", install_exe.display()))?;

    // Update user PATH
    let target_dir = install_dir.display().to_string();
    let user_path = env::var("PATH").unwrap_or_default();
    if !user_path.contains(&target_dir) {
        let current = Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                &format!(
                    "[Environment]::SetEnvironmentVariable('Path', $env:Path + ';{}', 'User')",
                    target_dir
                ),
            ])
            .status()
            .context("Failed to set user PATH via PowerShell")?;

        if current.success() {
            println!("✅ Added to user PATH: {}", target_dir);
        } else {
            println!("⚠️ Failed to update PATH. Please add it manually:");
            println!("{}", target_dir);
        }
    }

    println!("✅ Installed ssh-copy-id to:");
    println!("{}", install_exe.display());
    println!("You can now run `ssh-copy-id` from any new terminal!");

    Ok(())
}

fn uninstall_binary() -> Result<()> {
    let install_dir = data_local_dir()
        .context("Unable to resolve AppData\\Local\\Programs")?
        .join("ssh-copy-id");
    let install_exe = install_dir.join("ssh-copy-id.exe");

    if install_exe.exists() {
        fs::remove_file(&install_exe).context("Could not remove ssh-copy-id.exe")?;
    }

    if install_dir.exists() {
        fs::remove_dir_all(&install_dir).context("Could not remove install folder")?;
    }

    let path_cmd = r#"
$target = "$env:LocalAppData\Programs\ssh-copy-id"
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$newPath = ($userPath -split ';') | Where-Object { $_ -ne $target }
[Environment]::SetEnvironmentVariable("Path", ($newPath -join ';'), "User")
"#;

    let _ = Command::new("powershell")
        .args(["-NoProfile", "-Command", path_cmd])
        .status();

    println!("✅ Uninstalled ssh-copy-id.");

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.install {
        install_binary()?;
        exit(0);
    }

    if cli.uninstall {
        uninstall_binary()?;
        exit(0);
    }

    let pubkey_path = match &cli.key {
        Some(path) => path.clone(),
        None => get_default_pubkey_path()?,
    };

    let pubkey = read_pubkey(&pubkey_path)?;
    ssh_copy_id(&cli, &pubkey)?;

    Ok(())
}
