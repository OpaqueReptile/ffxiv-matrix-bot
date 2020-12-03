extern crate matrix_bot_api;
use matrix_bot_api::handlers::{
    HandleResult,
    Message,
    //MessageHandler,
    StatelessHandler,
};
use matrix_bot_api::{ActiveBot, MatrixBot, MessageType};

extern crate fractal_matrix_api;
use fractal_matrix_api::util::{media_url, put_media};

extern crate reqwest;
use url::Url;

// Just used for loading the username, password and homeserverurl from a file.
extern crate config;
// Just used for rolling dice
extern crate rand;

extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

//xivapi
use xivapi::error::ApiError;
use xivapi::models::content::Item;
use xivapi::models::id::ItemId;
use xivapi::models::search::StringAlgo::Fuzzy;
use xivapi::{
    models::search::{SearchModel, SearchResult},
    //models::character::{Race, Gender},
    //models::content::Item{}
    prelude::*,
};

trait JobClassId {
    fn to_jobclassid(&self) -> Result<&str, xivapi::error::ApiError>;
}

impl JobClassId for u64 {
    fn to_jobclassid(&self) -> Result<&str, xivapi::error::ApiError> {
        let ret = match self {
            14 => "ALC",
            10 => "ARM",
            9 => "BSM",
            //0 => "BTN",
            8 => "CRP",
            15 => "CUL",
            //0 => "FSH",
            11 => "GSM",
            12 => "LTW",
            //0 => "MIN",
            13 => "WVR",
            _ => {
                return Err(xivapi::error::ApiError {
                    error: true,
                    message: "Invalid JobClassId".to_string(),
                })
            }
        };
        Ok(ret)
    }
}

#[allow(unused_doc_comments)]
fn main() {
    // ------- Getting the login-credentials from file -------
    // You can get them however you like: hard-code them here, env-variable,
    // tcp-connection, read from file, etc. Here, we use the config-crate to
    // load from botconfig.toml.
    // Change this file to your needs, if you want to use this example binary.
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("botconfig"))
        .unwrap();

    let user = settings.get_str("user").unwrap();
    let password = settings.get_str("password").unwrap();
    let homeserver_url = settings.get_str("homeserver_url").unwrap();

    // -------------------------------------------------------
    // Define all the handlers
    let handler = StatelessHandler::new();

    /**
    //test handlers
    handler.register_handle("shutdown", |bot, _, _| {
        bot.shutdown();
        HandleResult::ContinueHandling /* Other handlers might need to clean up after themselves on shutdown */
    });

    handler.register_handle("echo", |bot, message, tail| {
        bot.send_message(
            &format!("Echo: {}", tail),
            &message.room,
            MessageType::TextMessage,
        );
        HandleResult::StopHandling
    });

    handler.register_handle("xivapi_test", |bot, message, _tail| {
        bot.send_message(&xivapi_test(), &message.room, MessageType::TextMessage);
        HandleResult::StopHandling
    });
    handler.register_handle("rainbow_test", |bot, message, _tail| {
        bot.send_html_message(
            "test",
            &rainbow_test(),
            &message.room,
            MessageType::TextMessage,
        );
        HandleResult::StopHandling
    });
    **/
    let mut roll = StatelessHandler::new();
    roll.register_handle("roll", roll_dice);
    roll.register_handle("help", roll_help);

    let mut item = StatelessHandler::new();
    item.register_handle("item", get_item);

    let marketboard = StatelessHandler::new();
    item.register_handle("marketboard", get_marketboard);
    item.register_handle("mb", get_marketboard);
    item.register_handle("market", get_marketboard);

    // -------------------------------------------------------
    // Start the bot
    let mut bot = MatrixBot::new(handler);
    bot.add_handler(roll);
    bot.add_handler(item);
    bot.add_handler(marketboard);
    bot.run(&user, &password, &homeserver_url);
}

fn roll_help(bot: &ActiveBot, message: &Message, _cmd: &str) -> HandleResult {
    let mut help = "Roll dice:\n".to_string();
    help += "!roll X [X ..]\n";
    help += "with\n";
    help += "X = some number. Thats the number of eyes your die will have.\n";
    help += "If multpile numbers are given, multiple dice are rolled. The result as a sum is displayed as well.\n";
    help += "\nExample: !roll 6 12 => Rolls 2 dice, one with 6, the other with 12 eyes.\n";
    bot.send_message(&help, &message.room, MessageType::RoomNotice);
    HandleResult::ContinueHandling /* There might be more handlers that implement "help" */
}

fn roll_dice(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let cmd_split = cmd.split_whitespace();

    let mut results: Vec<u32> = vec![];
    for dice in cmd_split {
        let sides = match dice.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                bot.send_message(
                    &format!("{} is not a number.", dice),
                    room,
                    MessageType::RoomNotice,
                );
                return HandleResult::StopHandling;
            }
        };
        results.push((rand::random::<u32>() % sides) + 1);
    }

    if results.len() == 0 {
        return roll_help(bot, message, cmd);
    }

    if results.len() == 1 {
        bot.send_message(&format!("{}", results[0]), room, MessageType::RoomNotice);
    } else {
        // make string from results:
        let str_res: Vec<String> = results.iter().map(|x| x.to_string()).collect();
        bot.send_message(
            &format!("{} = {}", str_res.join(" + "), results.iter().sum::<u32>()),
            room,
            MessageType::RoomNotice,
        );
    }

    HandleResult::StopHandling
}

fn item_header_msg(
    bot: &ActiveBot,
    item: &Item,
) -> Result<String, fractal_matrix_api::error::Error> {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("botconfig"))
        .unwrap();
    let name = item.other["Name"].as_str().unwrap();
    let icon = item.other["Icon"].as_str().unwrap();
    let rarity_color = match item.other["Rarity"].as_u64().unwrap() {
        1 => None,
        2 => Some("Green"),
        3 => Some("DeepSkyBlue"),
        4 => Some("DarkOrchid"),
        _ => Some("Pink"),
    };
    //name and item picture
    let name_html = match rarity_color {
        Some(color) => format!("<font color=\"{}\">{}</font>", color, name),
        _ => format!("{}", name),
    };
    let mut buf: Vec<u8> = vec![];
    reqwest::get(format!("https://xivapi.com{}", icon).as_str())
        .expect("request failed")
        .copy_to(&mut buf)
        .unwrap();
    match upload_file(
        bot.get_tk().as_str(),
        &Url::parse(settings.get_str("homeserver_url").unwrap().as_str()).unwrap(),
        buf,
    ) {
        Ok(mxc) => {
            println!("mxc for image is: {}", mxc);
            Ok(format!("<img style=\"vertical-align:middle\" src=\"{}\" alt=\"\" title=\"\" vertical-align=\"middle\" /><br>{}", mxc, name_html))
        }
        Err(e) => {
            println!("Unable to upload thumbnail");
            Err(e)
        }
    }
}
// Item Name (Color by rarity)
// Materia Slots <MateriaSlotCount>
fn get_item(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MarketData {
    dc_name: String,
    #[serde(rename = "itemID")]
    item_id: u64,
    last_upload_time: u64,
    listings: Vec<Listing>,
    #[serde(rename = "averagePriceNQ")]
    average_price_nq: f64,
    #[serde(rename = "averagePriceHQ")]
    average_price_hq: f64,
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
struct Listing {
    price_per_unit: u64,
    quantity: u64,
    //"retainerID":"eb11fac3a31ec7fbe57a89a283b17977969d7176c4917e38ca54ca1fa1d03102",
    //"sellerID":"455d3f412b33a6ceac22b1fedd8353da61b6e8ba63c5d715120f99185b6a0826",
    world_name: String,
    hq: bool,
}

fn get_marketboard(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
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
                                        "Lowest NQ: {} Gil on {}",
                                        nq_listings.first().unwrap().price_per_unit,
                                        nq_listings.first().unwrap().world_name
                                    )
                                        .to_string(),
                                );
                                optional_info.push(
                                    format!(
                                        "Highest NQ: {} Gil on {}\n",
                                        nq_listings.last().unwrap().price_per_unit,
                                        nq_listings.last().unwrap().world_name
                                    )
                                        .to_string(),
                                );
                            }
                            if !hq_listings.is_empty() {
                                optional_info.push(
                                    format!(
                                        "Lowest HQ: {} Gil on {}",
                                        hq_listings.first().unwrap().price_per_unit,
                                        hq_listings.first().unwrap().world_name
                                    )
                                        .to_string(),
                                );
                                optional_info.push(
                                    format!(
                                        "Highest HQ: {} Gil on {}\n",
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

fn upload_file(
    tk: &str,
    baseu: &Url,
    contents: Vec<u8>,
) -> Result<String, fractal_matrix_api::error::Error> {
    //let mut file = File::open(fname)?;
    //let mut contents: Vec<u8> = vec![];
    //file.read_to_end(&mut contents)?;

    let params = &[("access_token", tk.to_string())];
    let mediaurl = media_url(&baseu, "upload", params)?;
    println!("Uploading to {}", baseu);
    match put_media(mediaurl.as_str(), contents) {
        Err(err) => Err(err),
        Ok(js) => Ok(js["content_uri"].as_str().unwrap_or_default().to_string()),
    }
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

fn _xivapi_test() -> String {
    let api = XivApi::default();

    //let key = std::env::var("XIVAPI_KEY").unwrap();
    //let api = XivApi::with_key(&key);

    // let res = api
    //   .character_search()
    //   .name("Duvivi Duvi")
    //   .server(World::Adamantoise)
    //   .send()?;

    // let id = res.characters[0].id;

    // let res: CharInfoResult = api
    //   .character(1)
    //   .columns(&["Name", "Server", "Race", "Gender"])
    //   .json()?;

    //let res = api
    //    .character(1.into())
    //    .send().unwrap();
    //println!("{:#?}", res);

    // let res = api.enemy(7537.into()).send()?;
    // let res = api.character(2.into()).send()?;
    // let res = api.free_company_search().name("a").server(World::Adamantoise).send();
    // let res = api.free_company(9233645873504730768.into()).send();
    // let res = api.free_company(9233645873504776755.into()).send();
    // let res = api.linkshell_search()
    //   .name("lala world")
    //   .server(World::Adamantoise)
    //   .send();
    // let res = api.linkshell(20547673299957974.into()).send();

    //println!("{:#?}", res);

    //Ok(())

    //let res2 = api
    //    .item(10054.into())
    //    .send().unwrap();
    //println!("{:#?}", res2);

    //println!("{:#?}", api.search().string("aiming").send());
    let search: SearchResult = api
        .search()
        .string("Excalibur Zeta")
        .index(xivapi::models::search::Index::Item)
        .limit(2)
        .send()
        .unwrap();
    //serde_json::from_str::<ApiError>(search);

    format!("{:#?}", search.results)
}

fn _rainbow_test() -> String {
    return "<font color=\"#ff00be\">t</font><font color=\"#e0b500\">e</font><font color=\"#00e6b6\">s</font><font color=\"#00caff\">t</font>".to_string();
}
