use super::model;
use base64;
use reqwest;
use std::collections::HashSet;

pub struct RequestClient {
    port: String,
    token: String,
    client: reqwest::Client,
    auth: String,
}

impl RequestClient {
    pub fn new(port: String, token: String) -> RequestClient {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("failed to build reqwest client");

        let auth = format!("Basic {}", base64::encode(format!("riot:{}", token)));

        RequestClient {
            port: port,
            token: token,
            client: client,
            auth: auth,
        }
    }

    pub async fn get_chat_select_champ_id(&self) -> Option<String> {
        let url = format!("https://127.0.0.1:{}/lol-chat/v1/conversations", self.port);

        let response = self
            .client
            .get(url)
            .header("Authorization", &self.auth)
            .send()
            .await
            .expect("get_chat_select_champ_id: failed");

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<Vec<model::GameChatConversation>>().await {
                    Ok(parsed) => {
                        println!("Success! {:?}", parsed);
                        match parsed.iter().find(|c| c.r#type == "championSelect") {
                            Some(c) => {
                                return Some(c.id.clone());
                            }

                            None => {
                                return None;
                            }
                        }
                    }
                    Err(_) => {
                        println!("Uh oh! Something unexpected happened.");
                        return None;
                    }
                };
            }

            _ => {
                println!("Uh oh! Something unexpected happened.");
                return None;
            }
        }
    }

    pub async fn query_all_summoners_id(&self, id: String) -> Option<HashSet<String>> {
        let url = format!(
            "https://127.0.0.1:{}/lol-chat/v1/conversations/{}/messages",
            self.port, id
        );

        let response = self
            .client
            .get(url)
            .header("Authorization", &self.auth)
            .send()
            .await
            .expect("query_all_summoners_id: failed");

        match response.status() {
            reqwest::StatusCode::OK => {
                match response
                    .json::<Vec<model::GameChatConversationMessage>>()
                    .await
                {
                    Ok(parsed) => {
                        println!("Success! {:?}", parsed);
                        let ids = parsed.iter().map(|msg| msg.from_id.clone()).collect();

                        println!("All summoners' id queried.\n {:?}", &ids);
                        return Some(ids);
                    }
                    Err(_) => {
                        println!("Uh oh! Something unexpected happened.");
                        return None;
                    }
                };
            }

            _ => {
                println!("Uh oh! Something unexpected happened.");
                return None;
            }
        }
    }

    pub async fn get_summoner_recently_matches(
        &self,
        account_id: &str,
    ) -> Option<model::GameMatchList> {
        let url = format!(
            "https://127.0.0.1:{}/lol-match-history/v3/matchlist/account/{}",
            self.port, account_id
        );

        let response = self
            .client
            .get(url)
            .header("Authorization", &self.auth)
            .send()
            .await
            .expect("failed to get summoner recently matches");

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<model::GameMatchList>().await {
                    Ok(parsed) => {
                        println!("Success! {:?}", parsed);
                        return Some(parsed);
                    }
                    Err(_) => {
                        println!("Uh oh! Something unexpected happened.");
                        return None;
                    }
                };
            }

            _ => {
                println!("Uh oh! Something unexpected happened.");
                return None;
            }
        }
    }
}
