use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt};
use util::{client, model::GameQueryType};
mod util;

#[tokio::main]
async fn main() {
    let (port, token) = get_lol_config();
    let client = client::RequestClient::new(port, token);

    println!(
        "press 'r' to query rank info\n press 'j' to query polar chaos info\n press 'q' to quit."
    );
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "q" => {
                std::process::exit(0);
            }

            "r" => {
                analyse_horses(&client, GameQueryType::Rank).await;
                input.clear();
                println!("press 'r' to query rank info\n press 'j' to query polar chaos info\n press 'q' to quit.");
            }

            "j" => {
                analyse_horses(&client, GameQueryType::PolarChaos).await;
                input.clear();
                println!("press 'r' to query rank info\n press 'j' to query polar chaos info\n press 'q' to quit.");
            }

            _ => {
                input.clear();
                println!("press 'r' to query rank info\n press 'j' to query polar chaos info\n press 'q' to quit.");
            }
        }
    }
}

async fn analyse_horses(client: &client::RequestClient, game_query_type: GameQueryType) {
    let conversation_id = client.get_chat_select_champ_id().await;
    match conversation_id {
        Some(conversation_id) => {
            let ids = client.query_all_summoners_id(conversation_id).await;
            match ids {
                Some(ids) => {
                    let mut horses = Vec::new();
                    let mut horse_types = vec!["牛马", "大司马", "下等马", "中等马", "上等马"];

                    for id in ids {
                        let match_list = client.get_summoner_recently_matches(&id).await;
                        match match_list {
                            Some(match_list) => {
                                let summoner = match_list.get_summoner_name();
                                let avg_score =
                                    match_list.get_recently_rank_average_score(&game_query_type);
                                println!("马匹{}近期表现平均得分{}", summoner, avg_score);
                                horses.push((summoner.to_owned(), avg_score));
                            }

                            None => {
                                println!("获取召唤师马匹信息失败");
                            }
                        }
                    }
                    // analyse_horses
                    horses.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    println!("\n\n------------我方阵容组成---------------");
                    while let Some(horse) = horses.pop() {
                        if let Some(horse_type) = horse_types.pop() {
                            println!("{} {} 近期表现平均得分{}", horse_type, horse.0, horse.1);
                        }
                    }
                }

                None => {
                    println!("failed to query all summoners' id.");
                }
            }
        }

        None => {
            println!("failed to get covnersation_id.");
        }
    }
}

fn get_lol_config() -> (String, String) {
    let s = System::new_all();
    let lolc = s
        .processes_by_exact_name("LeagueClientUx.exe")
        .next()
        .expect("can't find process LeagueClientUx.exe");

    let command_line = lolc.cmd();
    if command_line.len() == 0 {
        panic!("League of Legends client rcommand line is null.");
    }
    let mut remoting_app_port = String::new();
    let mut auth_token = String::new();
    for command in command_line {
        if command.starts_with("--app-port=") {
            remoting_app_port = command.replace("--app-port=", "")
        } else if command.starts_with("--remoting-auth-token=") {
            auth_token = command.replace("--remoting-auth-token=", "");
        }
    }
    println!("port: {} token: {}", &remoting_app_port, &auth_token);
    (remoting_app_port, auth_token)
}
