
pub mod init;
pub mod check_exists;
pub mod make_conf;


pub fn route(args: &[String]) 
{
    match args.get(0).map(|s| s.as_str()) 
    {
        Some("check") => check_exists::run(),
        Some("init") => init::run(),
        _=> eprintln!("unknown wireg command."),
    }
}

