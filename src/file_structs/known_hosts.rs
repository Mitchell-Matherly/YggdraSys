use serde::Serialize;



#[derive(Serialize)]
pub struct Host {
    pub name: String,
    pub ip_addr: String,
    pub open_ports: Vec<String>,

}

#[derive(Serialize)]
pub struct HostDb {
    pub known_hosts: Vec<Host>,
}

