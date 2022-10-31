use serde::{Deserialize, Serialize};

// structs and stuff
#[derive(Serialize, Deserialize)]
pub struct config {
    pub BRIDGE: bridgedata,
    pub PLAYERS: Vec<player>,
}

impl Default for config {
    fn default() -> Self {
        config {
            BRIDGE: bridgedata {
                IP: String::from("None"),
                USER: String::from("None"),
            },
            PLAYERS: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct bridgedata {
    pub IP: String,
    pub USER: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct player {
    pub UUID: String,
    pub GROUPNAME: String,
    pub GROUPID: usize,
}
