use serde::Serialize;



#[derive(Serialize)]
pub struct ActiveProcesses {
    pub processes: Vec<Process>,
}

#[derive(Serialize)]
pub struct Process {
    pub name: String,
    pub status: String,
    pub current_provider: (String, String),
    pub list_of_hosts: Vec<String>,
}

