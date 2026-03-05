//This program checks for a wg install, then probes for a "wg0" interface. It will
//create/override by default, unless another name is supplied as an argument. 



use std::env;
use std::process::Command;

pub fn run()
{
    let args:Vec<String> = env::args().collect();


    if check_for_wg() == false { return }         //checks to see if /usr/bin/wg or
                                                            //usr/bin/wg-quick exists. It would be
                                                            //pretty hard to setup wireguard if it
                                                            //wasn't installed. 

}

fn check_for_wg() -> bool
{
    let check = Command::new("which")
        .arg("wg")
        .output()
        .unwrap();


    if check.status.success() 
    { 
        println!("WireGuard is installed."); 
        true
    }
    
    else 
    { 
        eprintln!("WireGuard is not installed, or not properly configured. Please ensure you have installed WireGuard and that the executable is located at /usr/bin/wg"); 
        false
    }
}
