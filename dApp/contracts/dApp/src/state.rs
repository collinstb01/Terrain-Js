use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

pub enum GameMove {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    HostWins,
    OpponentWins,
    Tie,
}

pub struct Start {
    pub host: Addr,
    pub opponent: Addr,
    pub host_move: GameMove, // enum GameMove { Rock, Paper, Scissors }
    pub opp_move: GameMove,
    pub result: GameResult,
}

pub const STATE: Item<State> = Item::new("state");
pub const ENTRIES: Map<&Addr, i32> = Map::new("entries");
pub const GAMESTATE: Map<Addr, Start> = Map::new("start");
