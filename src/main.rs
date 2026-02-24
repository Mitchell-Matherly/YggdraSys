//This program was created by Mitchell Matherly for the ConsenSys project. All liscences from the
//github repo apply.


mod wireg;
mod system;

use std::env;
use std::process;
use clap::{Arg, Command};

fn main() 
{

    let args:Vec<String> = env::args().collect();

    let matches = Command::new("consys")
        .version("0.1.0")
        .author("March Matherly")
        .about("ConsenSys manager utility")
        .subcommand(
            Command::new("system")
                .about("manage system settings for your machine")
                .subcommand(
                    Command::new("init")
                        .about("automatic, first-time setup for consys's system files.")
                            .arg(
                                Arg::new("role")
                                    .long("role")
                                    .value_name("ROLE")
                                    .help("chooses to enroll the host as a root or a branch. Follow with 'root' or 'branch.'")
                                    .required(true)
                                    .value_parser(["root", "branch"])
                            )               
                )
        )
        .subcommand(
            Command::new("wireg")
                .about("manage your connection to the wireguard config")
                .subcommand(
                    Command::new("init")
                        .about("automatic, first-time setup for consys's wireguard setup.")
                            .arg(
                                Arg::new("role")
                                    .long("role")
                                    .value_name("ROLE")
                                    .help("chooses to enroll the host as a root or a branch. follow with 'root' or 'branch.'")
                                    .required(true)
                                    .value_parser(["root", "branch"])
                            )
                )
        )
        .get_matches();

        match matches.subcommand() {
            

            Some (("system", system_m)) => {
                match system_m.subcommand() {
                    Some(("init", init_m)) => {
                        let role = init_m.get_one::<String>("role").unwrap();
                        match role.as_str() {
                            "root" => system::make_root_config(),
                            "branch" => system::make_branch_config(),
                            _=> { eprintln!("invalid input, please use 'root' or 'branch.'"); }

                        }
                    }
                    _=> { eprintln!("incorrect or no subcommand for system provided"); }
                }
            }
                
            Some (("wireg", wireg_m)) => {
                match wireg_m.subcommand() {
                    Some(("init", init_m)) => {
                       let role = init_m.get_one::<String>("role").unwrap();
                        match role.as_str() {
                            "root" => wireg::init(),
                            "branch" => wireg::init(),
                            _=> { eprintln!("invalid input, please use 'root' pr 'branch'") }
                        }
                    }
                    _=> { eprintln!("incorrect or no subcommand for system provided"); }
                }
            }

            _ => {
                eprintln!("invalid or no subcommand provided!");
                std::process::exit(1);
        
            }
        }
}

