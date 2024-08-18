use crate::config::Server;
use crate::protocol::StatusResponse;

pub fn display_response(response: &StatusResponse, server: &Server) {
    let players = &response.players;
    println!(
        "{} player(s) online on {}, address: {}:{}",
        players.online, server.alias,server.host, server.port
    );

    for player in &players.sample {
        println!("{}", player.name);
    }

    if players.online > players.sample.len() as i32 {
        println!("...");
    }
}

pub fn display_all_responses(all_responses: Vec<(StatusResponse, &Server)>) {
    for (response, server) in &all_responses {
        display_response(response, server);
    }
}
