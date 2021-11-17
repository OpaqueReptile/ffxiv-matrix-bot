extern crate config;
extern crate fractal_matrix_api;
extern crate matrix_bot_api;
extern crate rand;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate sled;

use matrix_bot_api::handlers::HandleResult;
use matrix_bot_api::{ActiveBot, Message, MessageType};
use xivapi::builder::Builder;
use xivapi::models::id::CharacterId;
use xivapi::XivApi;

static DBNAME: &str = "party.db";

//associate cmd user with character lodestone id, save to db
pub fn register_character(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let db: sled::Db = sled::open(DBNAME.clone()).unwrap();
    let api = XivApi::default();
    let room = &message.room;
    let user = &message.sender;
    let id = match cmd.trim_start_matches(" ").trim_end_matches(" ").parse() {
        Ok(id) => id,
        Err(e) => {
            bot.send_message(
                &format!(
                    "{} is an invalid character ID, kupo!",
                    cmd.to_string()
                        .trim_start_matches(" ")
                        .trim_end_matches(" ")
                ),
                room,
                MessageType::TextMessage,
            );
            println! {"{:#?}",e};
            return HandleResult::StopHandling;
        }
    };
    //get lodestone data
    let character = api.character(CharacterId(id)).send();
    println! {"{:#?}",character}

    HandleResult::StopHandling
}

pub fn join_party(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //join/create party, optional argument for party id
    HandleResult::StopHandling
}

pub fn leave_party(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //remove user from party, optional argument for party id
    HandleResult::StopHandling
}

pub fn delete_party(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //delete party and all members, optional argument for party id
    HandleResult::StopHandling
}

pub fn level_party(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //set current party to a min level, mainly used for rand_party
    HandleResult::StopHandling
}

pub fn set_role(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //set role to tank, healer, or dps
    HandleResult::StopHandling
}

pub fn rand_party(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    //based on party comp and assigned roles set people
    HandleResult::StopHandling
}
