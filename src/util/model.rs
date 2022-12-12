use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMatchList {
    pub accountId: usize,
    pub platformId: String,
    pub games: GameMatch,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameMatch {
    pub gameBeginDate: String,
    pub gameCount: usize,
    pub gameEndDate: String,
    pub gameIndexBegin: usize,
    pub gameIndexEnd: usize,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub gameId: usize,
    pub gameMode: String,
    pub gameType: String,
    pub gameVersion: String,
    pub mapId: usize,
    pub queueId: usize,
    pub participantIdentities: Vec<ParticipantIdentity>,
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
pub struct ParticipantIdentity {
    pub participantId: usize,
    pub player: ParticipantPlayer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipantPlayer {
    pub accountId: usize,
    pub summonerId: usize,
    pub summonerName: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Participant {
    pub championId: usize,
    pub highestAchievedSeasonTier: String,
    pub participantId: usize,
    pub stats: ParticipantStats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParticipantStats {
    pub assists: usize,
    pub causedEarlySurrender: bool,
    pub deaths: usize,
    pub kills: usize,
    pub doubleKills: usize,
    pub tripleKills: usize,
    pub quadraKills: usize,
    pub pentaKills: usize,
    pub killingSprees: usize,
    pub totalDamageDealtToChampions: usize,
    pub firstBloodAssist: bool,
    pub firstBloodKill: bool,
    pub win: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameChatConversation {
    pub id: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameChatConversationMessage {
    pub body: String,
    pub fromId: String,
    pub fromPid: String,
    pub fromSummonerId: usize,
    pub id: String,
    pub isHistorical: bool,
    pub timestamp: String,
    pub r#type: String,
}

impl ParticipantStats {
    pub fn get_match_score(&self) -> usize {
        let mut score = 100;
        if self.firstBloodKill {
            score += 10
        }
        if self.firstBloodAssist {
            score += 5
        }
        if self.causedEarlySurrender {
            score -= 10
        }
        if self.win {
            score += 5
        } else {
            score -= 5
        }
        score += self.doubleKills * 2;
        score += self.tripleKills * 5;
        score += self.quadraKills * 10;
        score += self.pentaKills * 15;
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
            .map(|game| &game.participantIdentities)
            .filter_map(|participants| participants.iter().next())
            .map(|p| &p.player.summonerName)
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
                game.gameMode == game_query_type.get_game_mode()
                    && game.gameType == game_query_type.get_game_type()
                    && game.queueId == game_query_type.get_queue_id()
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
