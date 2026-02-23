use super::check_exists;
use super::make_conf;


pub fn run()
{
    check_exists::run();
    println!("wireguard check run.");
    
    make_conf::run();
    println!("wireguard conf made.");
    

    println!("wireguard setup complete!");


}

