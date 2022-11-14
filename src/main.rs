use std::process::Command;
use util::client;
mod util;

#[tokio::main]
async fn main() {
    let (port, token) = get_port_and_token();
    let client = client::RequestClient::new(port, token);

    println!("press any key to query, press 'q' to quit.");
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "q" => {
                std::process::exit(0);
            }

            _ => {
                analyse_horses(&client).await;
                input.clear();
                println!("press any key to query, press 'q' to quit.");
            }
        }
    }
}

async fn analyse_horses(client: &client::RequestClient) {
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
                                let avg_score = match_list.get_recently_rank_average_score();
                                println!("马匹{}近期排位表现平均得分{}", summoner, avg_score);
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
                            println!("{} {} 近期排位表现平均得分{}", horse_type, horse.0, horse.1);
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

fn get_port_and_token() -> (String, String) {
    let output = Command::new("cmd")
        .args(&[
            "/C",
            "wmic PROCESS WHERE name='LeagueClientUx.exe' GET commandline",
        ])
        .output()
        .expect("wmic PROCESS command failed to start");

    let p_info = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", p_info);
    let port = p_info
        .split("--app-port=")
        .last()
        .expect("no app-port founded.")
        .split("\"")
        .next()
        .expect("no app-port founded.");

    let token = p_info
        .split("--remoting-auth-token=")
        .last()
        .expect("no remoting-auth-token founded.")
        .split("\"")
        .next()
        .expect("no remoting-auth-token founded.");

    println!("process info found.\n port:{}\n token:{}", port, token);

    (String::from(port), String::from(token))
}
