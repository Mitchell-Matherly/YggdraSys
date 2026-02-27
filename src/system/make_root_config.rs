//creates the basic file structures for ConsenSys to run. Config files in /etc/Ratatoskr, and
//state/identity files in /var/lib/Ratatoskr

use toml;
use serde::Serialize;
use serde::Deserialize;
use rand::RngExt;
use std::io;
use std::path::Path;
use std::env;
use std::env::args;
use std::fs;
use std::fs::File;



#[derive(Serialize)]
struct Config { system: System, network: Network }      //defines the structure for the Config file,
                                                        //composed of System and Netowkr parts.
    
#[derive(Serialize)]
struct System {                                         //System section in config files
    name: String, 
    role: String, 
    setup_complete: String 
    }
    
#[derive(Serialize)] 
struct Network {                                        //Network section in config files
    interface_name: String, 
    maximum_hosts: u16, 
    ip_addr: String, 
    public_key: String 
    }


pub fn make_root_config()
{

    let args: Vec<String> = env::args().collect();      
    let config_path = Path::new("/etc/Ratatoskr");      
    if !config_path.exists()                            //checks for an existing "/etc/Ratatoskr"
    {
        println!("no config files exist... creating.");
       
        make_config(config_path);
    }


    let runfiles_path = Path::new("/var/lib/Ratatoskr");
    if !runfiles_path.exists()                          //checks for existing "/var/lib/Ratatoskr"
    {
        make_runfiles(runfiles_path);

    }


    else { println!("auto config already completed. It's dangerous to run this again, so you'll have to go in manually. Otherwise, you could try deleting and reinstalling from stratch with ratskr init ragnarok."); }

//finish setting up file structure

}

fn make_config(config_path: &Path) -> io::Result<()> 
{ 
    fs::create_dir_all(config_path)?;
    println!("Created config directory at {}", config_path.display());

    let config_file_main = config_path.join("conf.toml");
    

    
    let mut rng = rand::rng();
    let n: u16 = rng.random_range(1..=999);
    let name_gen = format!("node {}", n);

    println!("this device is {}", name_gen);
    
    
    let config = Config {
        system: System {
            name: format!("node-{}",name_gen),
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
        .expect("failed to TOMLize");

    fs::write(config_file_main, tomlized_config_file)?;


    println!("config files created!");

    Ok(())
}

fn make_runfiles(runfiles_path: &Path) -> io::Result<()>
{

    fs::create_dir_all(runfiles_path);
    println!("Created runtime directory at {}", runfiles_path.display());

    
    let basepath = Path::new("/var/lib/ConsenSys");

    let identity = basepath.join("identity.toml");
    if !identity.exists()
    {
        println!("don't got no identity");

    }
    
    let activeprocesses = basepath.join("active.toml");
    if !activeprocesses.exists()
    {
        println!("don't got no activeprocess");

    }
    
    let activeprocesses = basepath.join("pubkey.json");
    if !activeprocesses.exists()
    {
        println!("don't got no pubkey");

    }
    
    Ok(())

}
