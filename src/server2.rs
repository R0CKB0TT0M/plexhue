use crate::jsonstructs::{bridgedata, config, player};
use hueclient::{self, Bridge, UnauthBridge};
use regex::Regex;
use rocket::Data;
use rocket::State;
use rocket::config::{Config, Environment};
use serde_json;
use serde_json::Value;
use std::io::Read;

#[post("/", format = "multipart/form-data", data = "<content>")]
fn webhook(config: State<config>, bridge: State<Bridge>, content: Data) {
    let re = Regex::new(r"\{(.*)\}").unwrap();
    let mut dataStr = "".to_string();
    content.open().read_to_string(&mut dataStr);
    let dataStr = re.captures(&dataStr).unwrap().get(0).unwrap().as_str();
    let jsondt: Value = serde_json::from_str(dataStr).unwrap();
    matchuuids(jsondt, bridge, config);
}
#[post("/", format = "multipart/form-data", data = "<content>")]
fn printplexuuids_helper(content: Data) {
    let re = Regex::new(r"\{(.*)\}").unwrap();
    let mut dataStr = "".to_string();
    content.open().read_to_string(&mut dataStr);
    let dataStr = re.captures(&dataStr).unwrap().get(0).unwrap().as_str();
    let jsondt: Value = serde_json::from_str(dataStr).unwrap();
    println!("Playeruuid: {}", jsondt["Player"]["uuid"]);
}

pub fn runserver(serverconfig: config, bridge: Bridge,port: Option<u16>) {
    
    let config = Config::build(Environment::Staging)
    .address("0.0.0.0")
    .port(port.unwrap_or(8000))
    .finalize().unwrap();

    rocket::custom(config)
        .manage(serverconfig)
        .manage(bridge)
        .mount("/", routes![webhook])
        .launch();
}

pub fn printplexuuids(port: Option<u16>) {

    let config = Config::build(Environment::Staging)
    .address("0.0.0.0")
    .port(port.unwrap_or(8000))
    .finalize().unwrap();
    println!("Start playing a video from your plex server now to see the uuid.(you will need this during setup)");
    
    rocket::custom(config)
        .mount("/", routes![printplexuuids_helper])
        .launch();
}

fn matchuuids(jsondt: Value, bridge: State<Bridge>, config: State<config>) {
    for player in &config.PLAYERS {
        if (jsondt["Player"]["uuid"] == player.UUID) {
            match jsondt["event"].as_str().unwrap() {
                "media.play" => {
                    let mut cmd = hueclient::CommandLight::default().off();
                    cmd.transitiontime = Some(40);

                    bridge.set_group_state(player.GROUPID, &cmd).unwrap();
                }
                "media.pause" => {
                    let mut cmd = hueclient::CommandLight::default().on().with_bri(178);
                    cmd.transitiontime = Some(25);

                    bridge.set_group_state(player.GROUPID, &cmd).unwrap();
                }
                "media.stop" => {
                    let mut cmd = hueclient::CommandLight::default().on().with_bri(178);
                    cmd.transitiontime = Some(25);

                    bridge.set_group_state(player.GROUPID, &cmd).unwrap();
                }
                "media.resume" => {
                    let mut cmd = hueclient::CommandLight::default().off();
                    cmd.transitiontime = Some(40);

                    bridge.set_group_state(player.GROUPID, &cmd).unwrap();
                }
                _ => {
                    println!("Received Unknown Event from Plex Server")
                }
            }
        }
    }
}
