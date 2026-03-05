//This file contains the structs for the config files, in /etc/Yggdrasys. It defines all the
//properties for each catagory, which allows you to simply load the files into a program as an
//object.
use serde::Serialize;


#[derive(Serialize)]
pub struct Config { pub system: System, pub network: Network }      

//currently, config coontains Systen and Network subcatagories. 

#[derive(Serialize)]
pub struct System {                                         
    pub name: String, 
    pub role: String, 
    pub setup_complete: String 
    }
    
//"name" contains the internal network name of the host that Yggdrasys is installed on.
//"role" dictates whether the host is set up as a branch or a root.
//"setup_complete" is a boolean that becomes "true" when setup scripts are completed.

#[derive(Serialize)] 
pub struct Network {                                       
    pub interface_name: String, 
    pub maximum_hosts: u16, 
    pub ip_addr: String, 
    pub public_key: String 
    }

//"interface_name" contains the name of the interface that Yggdrasys is using to connect to the
//interet. This should be configured to match your wireguard interface, otherwise your traffic will
//not be inside a VPN. The default name is wg0.
//"maximum_hosts" specifies the maximum hosts that can be on this network. for branches, this should
//not be changed. 
//"ip_addr" contains the internal ip address of the node. If this is working, probably don't change
//it.
//"public_key" contains your generated public key, used for connecting to wireguard VPNs. Don't
//share this randomly. 
