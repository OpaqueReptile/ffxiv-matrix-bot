extern crate matrix_bot_api;

use matrix_bot_api::ActiveBot;

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

use xivapi::models::content::Item;

use xivapi::{
    models::search::SearchResult,
    //models::character::{Race, Gender},
    //models::content::Item{}
    prelude::*,
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MarketData {
    pub dc_name: String,
    #[serde(rename = "itemID")]
    pub item_id: u64,
    pub last_upload_time: u64,
    pub listings: Vec<Listing>,
    #[serde(rename = "averagePriceNQ")]
    pub average_price_nq: f64,
    #[serde(rename = "averagePriceHQ")]
    pub average_price_hq: f64,
}

#[derive(Serialize, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub price_per_unit: u64,
    pub quantity: u64,
    //"retainerID":"eb11fac3a31ec7fbe57a89a283b17977969d7176c4917e38ca54ca1fa1d03102",
    //"sellerID":"455d3f412b33a6ceac22b1fedd8353da61b6e8ba63c5d715120f99185b6a0826",
    pub world_name: String,
    pub hq: bool,
}

pub trait JobClassId {
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

pub fn item_header_msg(
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
        2 => Some("#2ec685"), //Green, robinhoodish green
        3 => Some("#5091BF"), //Blue
        4 => Some("#ad5ad6"), //Purple, bit lighter than DarkOrchid
        _ => Some("#FF91A0"), //Pink
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

pub fn upload_file(
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
