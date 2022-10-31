#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use clap::{Parser, Subcommand};
use hueclient::{self, UnauthBridge, Bridge};
use serde::{Deserialize, Serialize};
use std::io;
use serde_json;
pub mod server2;
pub mod jsonstructs;
use jsonstructs::{config,bridgedata,player};

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::justip => {
            println!("{}", getbridge().ip);
        }
        Commands::server => {
            let config:config ={
            let text = std::fs::read_to_string("config.json").unwrap();
            serde_json::from_str(&text).unwrap()
            };
            let bridge =hueclient::Bridge::discover_required().with_user(&config.BRIDGE.USER.clone());
            server2::runserver(config,bridge,None); //Default port 8000. Custom Port Some(8000)
        }
        Commands::setup => {
            setup();
        }
        Commands::getuuid => {
            server2::printplexuuids(None);//Default port 8000. Custom Port Some(8000)
        }
        _ => {
            println!("command does not exist");
        }
    }
}

fn getbridge() -> UnauthBridge {
    let bridge = hueclient::Bridge::discover_required();
    bridge
}

fn setup() {
    let mut config = config::default();
    let bridge = getbridge();
    config.BRIDGE.IP = bridge.ip.to_string();
    let mut input = String::new();
    println!("Found Bridge!!! Press the Link button on your Bridge now to connect. Then press Enter/Return.");
    io::stdin().read_line(&mut input);
    let bridge = bridge.register_user("plexhueapp").unwrap();
    config.BRIDGE.USER= bridge.username.clone();
    config.PLAYERS.push(setup_player(&bridge).unwrap());
    println!("{}",serde_json::to_string(&config).unwrap());
    {let mut file = File::create("config.json").unwrap();
    write!(file,"{}",serde_json::to_string_pretty(&config).unwrap());
    println!("succesfully wrote to disk")
}}

fn setup_player(bridge:&Bridge) -> Result<player,&'static str> {
    let mut player_uuid = String::new();
    getplayeruuid(&mut player_uuid);
    let groups = bridge.get_all_groups().unwrap();
    println!("Found {} Groups", groups.len());
    for group in groups.iter(){
        println!("{}  : {}",group.id,group.group.name);
    }
    println!("Please enter the number of your group:");
    let mut groupid = String::new();
    io::stdin().read_line(&mut groupid);
    let groupid:usize = groupid.trim().parse::<usize>().unwrap();
    let mut groupname = String::new();
    for group in groups.iter(){
        if group.id == groupid {
            groupname = group.group.name.clone();
        }
    }
    let player = player{
        UUID : player_uuid,
        GROUPNAME : groupname,
        GROUPID : groupid,
    };
    Ok(player)
}

fn getplayeruuid(uuid:&mut String) {
    println!("please enter the uuid of your plex-player:");
    io::stdin().read_line( uuid);
    *uuid = String::from(uuid.trim());
}

//code ends here






#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    ///Return the IP of hue-bridge
    justip,
    ///Run the plexhue server
    server,
    ///Run the Setup
    setup,
    ///get plex player uuid
    getuuid,
}
