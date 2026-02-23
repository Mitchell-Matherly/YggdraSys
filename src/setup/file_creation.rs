//creates the basic file structures for ConsenSys to run. Config files in /etc/ConsenSys, and
//state/identity files in /var/lib/ConsenSys

use std::path::Path

fn run()
{
    let config_path = Path::new("/etc/serverus");
    if !config_path.exists() 
    {
        fs::create_dir_all(path)?;
        println!("Created config directory at {}", config_path.display());
    }


    let runfiles_path = Path::new("/var/lib/serverus");
    if !runfiles_path.exists()
    {
        fs::create_dir_all(path)?;
        println!("Created runtime directory at {}", runfiles_path.display();
    }


    
//finish setting up file structure







}
