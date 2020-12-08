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