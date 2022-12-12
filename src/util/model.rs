use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameMatchList {
    pub account_id: usize,
    pub platform_id: String,
    pub games: GameMatch,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameMatch {
    pub game_begin_date: String,
    pub game_count: usize,
    pub game_end_date: String,
    pub game_index_begin: usize,
    pub game_index_end: usize,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_id: usize,
    pub game_mode: String,
    pub game_type: String,
    pub game_version: String,
    pub map_id: usize,
    pub queue_id: usize,
    pub participant_identities: Vec<ParticipantIdentity>,
    pub participants: Vec<Participant>,
}

pub enum GameQueryType {
    Rank,
    PolarChaos,
}

impl GameQueryType {
    pub fn get_queue_id(&self) -> usize {
        match self {
            GameQueryType::Rank => 420,
            GameQueryType::PolarChaos => 450,
        }
    }

    pub fn get_game_mode(&self) -> &str {
        match self {
            GameQueryType::Rank => "CLASSIC",
            GameQueryType::PolarChaos => "ARAM",
        }
    }

    pub fn get_game_type(&self) -> &str {
        match self {
            GameQueryType::Rank => "MATCHED_GAME",
            GameQueryType::PolarChaos => "MATCHED_GAME",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    pub participant_id: usize,
    pub player: ParticipantPlayer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantPlayer {
    pub account_id: usize,
    pub summoner_id: usize,
    pub summoner_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub champion_id: usize,
    pub highest_achieved_season_tier: String,
    pub participant_id: usize,
    pub stats: ParticipantStats,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantStats {
    pub assists: usize,
    pub caused_early_surrender: bool,
    pub deaths: usize,
    pub kills: usize,
    pub double_kills: usize,
    pub triple_kills: usize,
    pub quadra_kills: usize,
    pub penta_kills: usize,
    pub killing_sprees: usize,
    pub total_damage_dealt_to_champions: usize,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameChatConversation {
    pub id: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameChatConversationMessage {
    pub body: String,
    pub from_id: String,
    pub from_pid: String,
    pub from_summoner_id: usize,
    pub id: String,
    pub is_historical: bool,
    pub timestamp: String,
    pub r#type: String,
}

impl ParticipantStats {
    pub fn get_match_score(&self) -> usize {
        let mut score = 100;
        if self.first_blood_kill {
            score += 10
        }
        if self.first_blood_assist {
            score += 5
        }
        if self.caused_early_surrender {
            score -= 10
        }
        if self.win {
            score += 5
        } else {
            score -= 5
        }
        score += self.double_kills * 2;
        score += self.triple_kills * 5;
        score += self.quadra_kills * 10;
        score += self.penta_kills * 15;
        score += self.assists;
        score += self.kills * 2;
        score -= self.deaths;
        score
    }
}

impl GameMatchList {
    pub fn get_summoner_name(&self) -> &str {
        let name = self
            .games
            .games
            .iter()
            .map(|game| &game.participant_identities)
            .filter_map(|participants| participants.iter().next())
            .map(|p| &p.player.summoner_name)
            .next()
            .expect("no summoner's name");

        &name
    }

    pub fn get_recently_rank_average_score(&self, game_query_type: &GameQueryType) -> f32 {
        let scores: Vec<usize> = self
            .games
            .games
            .iter()
            .filter(|game| {
                game.game_mode == game_query_type.get_game_mode()
                    && game.game_type == game_query_type.get_game_type()
                    && game.queue_id == game_query_type.get_queue_id()
            })
            .map(|game| &game.participants)
            .filter_map(|participants| participants.iter().next())
            .map(|p| p.stats.get_match_score())
            .collect();

        if scores.len() == 0 {
            return 95.0;
        }

        let avg_score = scores.iter().sum::<usize>() as f32 / scores.len() as f32;
        avg_score
    }
}
