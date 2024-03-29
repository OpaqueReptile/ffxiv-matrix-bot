#![allow(dead_code)]
#![allow(unused_variables)]

extern crate matrix_bot_api;
use matrix_bot_api::handlers::StatelessHandler;
use matrix_bot_api::MatrixBot;

extern crate fractal_matrix_api;

extern crate reqwest;

// Just used for loading the username, password and homeserverurl from a file.
extern crate config;
// Just used for rolling dice
extern crate rand;

extern crate serde;
extern crate serde_json;

//xivapi

//load submodules
mod countdown;
mod item;
mod party;
mod random;
mod status;
mod util;

fn main() {
    main_bot();
    //main_test();
}

fn main_test() {
    print!("{}", status::status_message())
}

#[allow(unused_doc_comments)]
fn main_bot() {
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

    /*
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
    */
    let mut ffxiv_handle = StatelessHandler::new();
    ffxiv_handle.register_handle("item", item::get_item);
    ffxiv_handle.register_handle("marketboard", item::get_marketboard);
    ffxiv_handle.register_handle("mb", item::get_marketboard);
    ffxiv_handle.register_handle("market", item::get_marketboard);
    ffxiv_handle.register_handle("roll", random::roll_dice);
    ffxiv_handle.register_handle("help", random::roll_help);
    ffxiv_handle.register_handle("register", party::register_character);
    ffxiv_handle.register_handle("countdown", countdown::countdown);
    //ffxiv_handle.register_handle("countdown_nano", countdown::countdown_nano);
    ffxiv_handle.register_handle("countdown_loop", countdown::countdown_loop);
    ffxiv_handle.register_handle("spoiler", status::status);
    ffxiv_handle.register_handle("status", status::status);
    ffxiv_handle.register_handle("status_loop", status::status_loop);

    // -------------------------------------------------------
    // Start the bot
    let mut bot = MatrixBot::new(handler);
    bot.add_handler(ffxiv_handle);
    bot.run(&user, &password, &homeserver_url);
}
