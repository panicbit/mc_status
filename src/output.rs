use anyhow::Result;

use crate::config::Server;
use crate::protocol::StatusResponse;

pub fn display_response_result(server: &Server, response_result: Result<StatusResponse>) {
    let response = match response_result {
        Ok(response) => response,
        Err(err) => {
            println!("Error getting status for {} | {:?}\n", server.alias, err);
            return;
        }
    };

    let players = &response.players;

    println!(
        "{} player(s) online on {} | IP: {}:{}",
        players.online, server.alias, server.host, server.port
    );

    for player in &players.sample {
        println!("{}", player.name);
    }

    if players.online > players.sample.len() as i32 {
        println!("...");
    }
}
