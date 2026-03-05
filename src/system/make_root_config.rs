//creates the basic file structures for Yggdrasys to run. Config files in /etc/Yggdrasys, and
//state/identity files in /var/lib/Yggdrasys

use toml;
use rand::RngExt;
use std::io;
use std::path::Path;
use std::env;
use std::fs;

use crate::file_structs::{
    config::Config,
    config::System,
    config::Network,
    activeprocesses::ActiveProcesses,
    identity::Identity,
    known_hosts::Host,
    known_hosts::HostDb
};




pub fn make_root_config()
{

    let args: Vec<String> = env::args().collect();
    
    let mut rng = rand::rng();
    let n: u16 = rng.random_range(1..=999);
    let name_gen = format!("root-{}", n);               //generate a temporary random name for the
                                                        //host, to be used if files are not found.
    

    let config_path = Path::new("/etc/Yggdrasys");      
    if !config_path.exists()                            //checks for an existing "/etc/Yggdrasys"
    {
        println!("no config files exist... creating.");
       
        make_config(config_path, name_gen.clone());
    }


    let runfiles_path = Path::new("/var/lib/Yggdrasys");
    if !runfiles_path.exists()                          //checks for existing "/var/lib/Yggdrasys"
    {
        make_runfiles(runfiles_path, name_gen.clone());

    }


    else { println!("auto config already completed. It's dangerous to run this again, so you'll have to go in manually. Otherwise, you could try deleting and reinstalling from stratch with ratskr init ragnarok."); }

//finish setting up file structure

}

fn make_config(config_path: &Path, name_gen: String) -> io::Result<()> 
{ 
    fs::create_dir_all(config_path)?;
    println!("Created config directory at {}", config_path.display());

    let config_file_main = config_path.join("conf.toml");
    

    

    println!("this device is {}", name_gen);
    
    
    let config = Config {
        system: System {
            name: format!("{}",name_gen),
            role: "root".to_string(),
            setup_complete: "no".to_string()
        },
        network: Network {
            interface_name: "wg0".to_string(),
            maximum_hosts: 24,
            ip_addr: "0.0.0.0/24".to_string(),
            public_key: "000-000-000".to_string(),
        },
    };

    let tomlized_config_file = toml::to_string_pretty(&config)
        .expect("failed to TOMLize config.toml");

    fs::write(config_file_main, tomlized_config_file)?;


    println!("config files created!");

    Ok(())
}

fn make_runfiles(runfiles_path: &Path, name_gen: String) -> io::Result<()>
{

    fs::create_dir_all(runfiles_path);
    println!("Created runtime directory at {}", runfiles_path.display());

    
    let basepath = Path::new("/var/lib/Yggdrasys");

    let identity_path = basepath.join("identity.toml");
    if !identity_path.exists()
    {
        let identity = Identity {
            name: format!("{}", name_gen),
        };

        let tomlized_identity_file = toml::to_string_pretty(&identity)
            .expect("Failed to TOMLize identity.toml");
    
        fs::write(identity_path, tomlized_identity_file)?;

        println!("Identity.toml created");

    } 
    let activeprocesses = basepath.join("active.toml");
    if !activeprocesses.exists()
    {
        println!("don't got no activeprocess");

    }
    
    let pubkey = basepath.join("pubkey.json");
    if !activeprocesses.exists()
    {
        println!("don't got no pubkey");

    }
    
    Ok(())

}
