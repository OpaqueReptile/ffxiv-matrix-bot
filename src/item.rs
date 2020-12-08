extern crate matrix_bot_api;
use matrix_bot_api::handlers::{HandleResult, Message};
use matrix_bot_api::{ActiveBot, MessageType};

extern crate fractal_matrix_api;

extern crate reqwest;

// Just used for loading the username, password and homeserverurl from a file.
extern crate config;
// Just used for rolling dice
extern crate rand;

extern crate serde;
extern crate serde_json;

//xivapi
use xivapi::error::ApiError;

use xivapi::models::id::ItemId;
use xivapi::models::search::StringAlgo::Fuzzy;
use xivapi::{models::search::SearchModel, prelude::*};

use crate::util::*;

// Item Name (Color by rarity)
// Materia Slots <MateriaSlotCount>
pub fn get_item(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let api = XivApi::default();
    let room = &message.room;
    let _cmd_split = cmd.split_whitespace();
    let mut _results: Vec<u32> = vec![];
    match xivapi_find_id(cmd.to_string()) {
        Ok(id) => {
            let item = api.item(id).send().unwrap();
            let name = item.other["Name"].as_str().unwrap();
            let name_html = item_header_msg(bot, &item).unwrap();

            // Iterate through optional info
            // Type: <ClassJobUse> + <ItemType>
            // <Unique> <Untradable>
            // Required level: <LevelEquip>
            // Item Level: <LevelItem>
            // Craftable? - Yes/No by <class>
            let mut optional_info = vec![];

            match item.other.get("IsUnique") {
                Some(unique) => match unique.as_i64() {
                    Some(0) => optional_info.push("Unique - ❌".to_string()),
                    Some(1) => optional_info.push("Unique - ✅".to_string()),
                    _ => panic!("Invalid IsUnique value"),
                },
                _ => (),
            };
            match item.other.get("IsUntradable") {
                Some(tradable) => match tradable.as_i64() {
                    Some(1) => optional_info.push("Tradable - ❌".to_string()),
                    Some(0) => optional_info.push("Tradable - ✅".to_string()),
                    _ => panic!("Invalid IsUntradable value"),
                },
                _ => (),
            };
            match item.other.get("LevelEquip") {
                Some(level) if level.as_u64().unwrap() > 1 => {
                    optional_info.push(format!("Required Level: {}", level).to_string())
                }
                _ => (),
            };
            match item.other.get("LevelItem") {
                Some(item_level) => {
                    optional_info.push(format!("Item Level: {}", item_level).to_string())
                }
                _ => (),
            };
            match item.other.get("Recipes") {
                Some(craftable) => {
                    let mut msg = String::from("\nCraftable by:\n  ");
                    match craftable.as_array() {
                        Some(array) => {
                            for obj in array {
                                msg = format!(
                                    "{}{} at lvl {}\n  ",
                                    msg,
                                    obj.as_object()
                                        .unwrap()
                                        .get("ClassJobID")
                                        .unwrap()
                                        .as_u64()
                                        .unwrap()
                                        .to_jobclassid()
                                        .unwrap(),
                                    obj.as_object()
                                        .unwrap()
                                        .get("Level")
                                        .unwrap()
                                        .as_u64()
                                        .unwrap()
                                )
                            }
                        }
                        _ => (),
                    }
                    optional_info.push(msg);
                }
                _ => (),
            };
            match item.other.get("Description") {
                Some(desc) => optional_info.push(format!("{}", desc).to_string()),
                _ => (),
            };
            println!("optional_info: {:#?}", optional_info);
            let mut info_msg = String::from("");
            for info in optional_info {
                info_msg = format!("{}{}\n", info_msg, info);
            }

            //Bulid the final message, formatted and unformatted
            let greeting = format!("Found it, kupo!");
            let final_msg = format!("{}\n\n{}\n\n{}", greeting, name, info_msg);
            let final_html_msg = format!(
                "{}<br><br>{}<br><br>{}",
                greeting,
                name_html,
                info_msg.replace("\n", "<br>")
            );
            bot.send_html_message(&final_msg, &final_html_msg, room, MessageType::TextMessage)
        }
        Err(_) => bot.send_message(
            &format!(
                "Unable to find an item called {}, kupo!",
                cmd.to_string()
                    .trim_start_matches(" ")
                    .trim_end_matches(" ")
            ),
            room,
            MessageType::TextMessage,
        ),
    }
    HandleResult::StopHandling
}

fn xivapi_find_id(item: String) -> Result<ItemId, ApiError> {
    let api = XivApi::default();
    let search = match api
        .search()
        .string(&item.trim_start_matches(" ").trim_end_matches(" "))
        .string_algo(Fuzzy)
        .index(xivapi::models::search::Index::Item)
        .limit(5)
        .send()
    {
        Ok(result) => {
            println!("{:#?}", result.results);
            result
        }
        Err(_e) => {
            return Err(xivapi::error::ApiError {
                error: true,
                message: "Invalid Search".to_string(),
            })
        }
    };
    match search.results.first() {
        Some(SearchModel::Item(item)) => Ok(ItemId(item.metadata.id)),
        _ => Err(ApiError {
            error: true,
            message: "Unable to find item!".to_string(),
        }),
    }
}

pub(crate) fn get_marketboard(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let api = XivApi::default();
    let room = &message.room;
    let _cmd_split = cmd.split_whitespace();
    let mut _results: Vec<u32> = vec![];
    match xivapi_find_id(cmd.to_string()) {
        Ok(id) => {
            let item = api.item(id).send().unwrap();
            let name = item.other["Name"].as_str().unwrap();
            let name_html = item_header_msg(bot, &item).unwrap();
            match reqwest::get(format!("https://universalis.app/api/Aether/{}", id.0).as_str()) {
                Ok(mut res) => {
                    match serde_json::from_str(res.text().unwrap().as_str()) {
                        Ok(d) => {
                            let data: MarketData = d;
                            let mut optional_info = vec![];
                            optional_info.push(
                                format!("Average NQ: {} Gil", data.average_price_nq as u64)
                                    .to_string(),
                            );
                            if data.average_price_hq as u64 > 0 {
                                optional_info.push(
                                    format!("Average HQ: {} Gil\n", data.average_price_hq as u64)
                                        .to_string(),
                                );
                            }
                            let mut hq_listings: Vec<Listing> =
                                data.listings.clone().into_iter().filter(|l| l.hq).collect();
                            let mut nq_listings: Vec<Listing> = data
                                .listings
                                .clone()
                                .into_iter()
                                .filter(|l| !l.hq)
                                .collect();
                            hq_listings.sort();
                            nq_listings.sort();
                            if !nq_listings.is_empty() {
                                optional_info.push(
                                    format!(
                                        "Lowest NQ: {}x {} Gil on {}",
                                        nq_listings.first().unwrap().quantity,
                                        nq_listings.first().unwrap().price_per_unit,
                                        nq_listings.first().unwrap().world_name
                                    )
                                    .to_string(),
                                );
                                optional_info.push(
                                    format!(
                                        "Highest NQ: {}x {} Gil on {}\n",
                                        nq_listings.last().unwrap().quantity,
                                        nq_listings.last().unwrap().price_per_unit,
                                        nq_listings.last().unwrap().world_name
                                    )
                                    .to_string(),
                                );
                            }
                            if !hq_listings.is_empty() {
                                optional_info.push(
                                    format!(
                                        "Lowest HQ: {}x {} Gil on {}",
                                        hq_listings.first().unwrap().quantity,
                                        hq_listings.first().unwrap().price_per_unit,
                                        hq_listings.first().unwrap().world_name
                                    )
                                    .to_string(),
                                );
                                optional_info.push(
                                    format!(
                                        "Highest HQ: {}x {} Gil on {}\n",
                                        hq_listings.last().unwrap().quantity,
                                        hq_listings.last().unwrap().price_per_unit,
                                        hq_listings.last().unwrap().world_name
                                    )
                                    .to_string(),
                                );
                            }
                            println!("optional_info: {:#?}", optional_info);
                            let mut info_msg = String::from("");
                            for info in optional_info {
                                info_msg = format!("{}{}\n", info_msg, info);
                            }

                            let greeting =
                                format!("Here's how the markets look for {}, kupo!", name);
                            let final_msg = format!("{}\n\n{}\n\n{}", greeting, name, info_msg);
                            let final_html_msg = format!(
                                "{}<br><br>{}<br><br>{}",
                                greeting,
                                name_html,
                                info_msg.replace("\n", "<br>")
                            );
                            bot.send_html_message(
                                &final_msg,
                                &final_html_msg,
                                room,
                                MessageType::TextMessage,
                            )
                        }
                        Err(_) => bot.send_message(
                            &format!("Doesn't look like {} is for sale, kupo!", name.to_string()),
                            room,
                            MessageType::TextMessage,
                        ),
                    };
                }
                _ => (),
            }
        }
        Err(_) => bot.send_message(
            &format!(
                "Unable to find an item called {}, kupo!",
                cmd.to_string()
                    .trim_start_matches(" ")
                    .trim_end_matches(" ")
            ),
            room,
            MessageType::TextMessage,
        ),
    };
    HandleResult::StopHandling
}
