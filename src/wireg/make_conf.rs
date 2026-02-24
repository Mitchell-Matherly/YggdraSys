//checks for a wireguard config file (wg0) and if it exists, backs it up and creates a new one. If it doesn't exist, creates one.

use std::path::Path;
use std::fs;
use std::fs::File;
use std::io;
use std::process::Command;
use std::process::Stdio;
use std::io::Write;


pub fn run() -> io::Result<()>
{
 
    let confpath = Path::new("/etc/wireguard/wg0.conf");
    let bakpath = Path::new("/etc/wireguard/bak.wg0.conf");



    if confpath.exists() == true 
    {
        fs::rename(confpath, bakpath)?;
        println!("moved existing wg0 to bak.wg0.conf");
    }

    let mut confile = File::create(confpath)?;
    println!("Created {}", confpath.display());


    let private_key_output = Command::new("wg")
        .arg("genkey")
        .output()?;

    let private_key = String::from_utf8(private_key_output.stdout)
        .expect("Failed to parse key")
        .trim()
        .to_string();
    println!("Private Key: {}", private_key);

    let mut pubkey_process = Command::new("wg")
        .arg("pubkey")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    {
        let stdin = pubkey_process.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(private_key.as_bytes())?;
    }



    let output = pubkey_process.wait_with_output()?;
    let public_key = String::from_utf8(output.stdout)
        .expect("Failed to parse public key")
        .trim()
        .to_string();

    let default_config = format!(
    r#"[Interface]
    Address = 10.0.0.1/24
    ListenPort = 51820
    #DO NOT SHARE YOUR PRIVATE KEY
    PrivateKey = {}
    
    #PublicKey (share this) = {}
   

    "#,
        private_key, public_key
    );

    confile.write_all(default_config.as_bytes())?;
    println!("Success! wrote wg0 file.");


    Ok(())



}
