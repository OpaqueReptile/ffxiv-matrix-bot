extern crate matrix_bot_api;
use matrix_bot_api::handlers::{HandleResult, Message};
use matrix_bot_api::{ActiveBot, MessageType};
extern crate fractal_matrix_api;

extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
use self::sheets4::api::{CellData, ExtendedValue};
use sheets4::Error;
use sheets4::Sheets;
use std::thread;

//use std::fs::File;
//use std::io::Write;
//use url::OpaqueOrigin;
//use futures::executor::block_on;

//xivapi

static LEVEL_FLOOR: u32 = 80;

#[derive(Clone)]
enum MsqLevelStatus {
    MSQInProgress,
    WaitingOnTrial,
    WaitingOnDungeon,
    MSQComplete,
    None,
    //...
}

impl MsqLevelStatus {
    pub fn from(val: &str) -> MsqLevelStatus {
        match val {
            "MSQ In Progress" => MsqLevelStatus::MSQInProgress,
            "Waiting On Trial" => MsqLevelStatus::WaitingOnTrial,
            "Waiting On Dungeon" => MsqLevelStatus::WaitingOnDungeon,
            "All MSQ Quest Complete" => MsqLevelStatus::MSQComplete,
            _ => MsqLevelStatus::None,
        }
    }
}

impl From<&str> for MsqLevelStatus {
    fn from(val: &str) -> MsqLevelStatus {
        match val {
            "MSQ In Progress" => MsqLevelStatus::MSQInProgress,
            "Waiting On Trial" => MsqLevelStatus::WaitingOnTrial,
            "Waiting On Dungeon" => MsqLevelStatus::WaitingOnDungeon,
            "All MSQ Quest Complete" => MsqLevelStatus::MSQComplete,
            _ => MsqLevelStatus::None,
        }
    }
}
impl From<&MsqLevelStatus> for String {
    fn from(val: &MsqLevelStatus) -> String {
        match val {
            MsqLevelStatus::MSQInProgress => "MSQ In Progress".to_string(),
            MsqLevelStatus::WaitingOnTrial => "Waiting On Trial".to_string(),
            MsqLevelStatus::WaitingOnDungeon => "Waiting On Dungeon".to_string(),
            MsqLevelStatus::MSQComplete => "All MSQ Quest Complete".to_string(),
            _ => "None".to_string(),
        }
    }
}

#[derive(Clone)]
struct Row {
    char_name: String,
    statuses: Vec<MsqLevelStatus>,
    // ...
}

impl Row {
    pub fn from(row_data: google_sheets4::api::RowData) -> Self {
        let nullrow = Row {
            char_name: "".to_string(),
            statuses: vec!["None".into()],
        };
        match row_data.values {
            None => nullrow,
            Some(cell_vec) => {
                let char_cell: CellData = cell_vec[0].clone();
                match char_cell.effective_value {
                    None => nullrow,
                    Some(char_exval) => {
                        let char_string: String = match char_exval.string_value {
                            Some(somestring) => somestring.to_string(),
                            None => "".to_string()
                        };
                        if char_string.len() > 0 {
                            let status_cells: Vec<CellData> = cell_vec[1..].to_vec().clone();
                            let status_exvals: Vec<ExtendedValue> = status_cells
                                .into_iter()
                                .map(|cd| cd.effective_value.unwrap())
                                .collect();
                            let status_enums: Vec<MsqLevelStatus> = status_exvals
                                .into_iter()
                                .map(|ex| ex.string_value.unwrap().as_str().into())
                                .collect();
                            Row {
                                char_name: char_string,
                                statuses: status_enums,
                            }
                        } else {
                            Row {
                                char_name: char_string,
                                statuses: Vec::new(),
                            }
                        }
                    }
                }
            }
        }
    }
    pub fn highest_complete(&self) -> Option<u32> {
        let mut highest: isize = -1;
        for (i, status) in self.statuses.iter().enumerate() {
            if matches!(status, MsqLevelStatus::MSQComplete) {
                highest = i as isize;
            }
            //println!("At {} highest is {} and status {:?}", i, highest, String::from(status));
        }
        match highest {
            -1 => None,
            _ => Some(highest as u32),
        }
    }

    pub fn highest_duty(&self) -> Option<(MsqLevelStatus, u32)> {
        let mut duty = MsqLevelStatus::None;
        let mut level = 0;
        for (i, status) in self.statuses.iter().enumerate() {
            if matches!(status, MsqLevelStatus::WaitingOnDungeon)
                || matches!(status, MsqLevelStatus::WaitingOnTrial)
            {
                duty = status.clone();
                level = i;
            }
            //println!("At {} highest is {} and status {:?}", i, highest, String::from(status));
        }
        match duty {
            MsqLevelStatus::None => None,
            _ => Some((duty, level as u32)),
        }
    }

    pub fn highest_prog(&self) -> Option<(MsqLevelStatus, u32)> {
        let mut duty = MsqLevelStatus::None;
        let mut level = 0;
        for (i, status) in self.statuses.iter().enumerate() {
            if matches!(status, MsqLevelStatus::MSQInProgress) {
                duty = status.clone();
                level = i;
            }
            //println!("At {} highest is {} and status {:?}", i, highest, String::from(status));
        }
        match duty {
            MsqLevelStatus::None => None,
            _ => Some((duty, level as u32)),
        }
    }
}

#[derive(Clone)]
struct PeopleAtLevel {
    level: i32,
    char_names: Vec<String>,
    // ...
}

#[derive(Clone)]
struct PeopleAtDuty {
    level: Vec<i32>,
    duty: Vec<MsqLevelStatus>,
    char_names: Vec<String>,
    // ...
}

#[tokio::main]
async fn get_status_rows() -> Vec<Row> {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret = yup_oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("client secret could not be read");
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    let hub = Sheets::new(
        hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
        auth,
    );

    let result = hub
        .spreadsheets()
        .get("1hnq73FbH2pwKwqeALePNuL22H-2ipSl0JuDciN9RTkA") // your spreadsheet id enters here
        .add_ranges("MSQ Progress")
        .include_grid_data(true)
        .doit()
        .await;

    // println!("{:?}",result);
    let nullrow = Row {
        char_name: "".to_string(),
        statuses: vec!["None".into()],
    };
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
            | Error::Io(_)
            | Error::MissingAPIKey
            | Error::MissingToken(_)
            | Error::Cancelled
            | Error::UploadSizeLimitExceeded(_, _)
            | Error::Failure(_)
            | Error::BadRequest(_)
            | Error::FieldClash(_)
            | Error::JsonDecodeError(_, _) => {
                println!("{}", e);
                vec![nullrow]
            }
        },
        Ok(res) => {
            let g = res
                .1
                .sheets
                .unwrap()
                .pop()
                .unwrap()
                .data
                .unwrap()
                .pop()
                .unwrap();
            let rows: Vec<Row> = g
                .row_data
                .unwrap()
                .into_iter()
                .map(|r| Row::from(r))
                .collect();
            rows
        }
    }
}

fn get_everyone_completed_level(rows: &Vec<Row>) -> Option<(u32, u32)> {
    let mut people_count = 0;
    let mut complete_count = vec![0; 11];
    for row in rows {
        if !row.char_name.is_empty() {
            people_count += 1;
            print!("Person: {} ", row.char_name);
            match row.highest_complete() {
                Some(level) => {
                    print!("has completed levels: ");
                    for i in 0..=level {
                        complete_count[i as usize] += 1;
                        print!("{} ", i);
                    }
                }
                None => (),
            }
            println!();
        }
    }
    println!("There are {} people", people_count);
    println!("Scores: {:?}", complete_count);
    let mut highest: isize = -1;
    for (i, count) in complete_count.iter().enumerate() {
        if *count as i32 == people_count as i32 {
            highest = i as isize;
            print!("{} ", i);
        }
    }
    match highest {
        -1 => None,
        _ => Some((highest as u32, people_count as u32)),
    }
}

fn get_furthest_level(rows: &Vec<Row>) -> Option<PeopleAtLevel> {
    let mut furthest = PeopleAtLevel {
        char_names: Vec::new(),
        level: -1,
    };
    for row in rows {
        if !row.char_name.is_empty() {
            print!("Person: {} ", row.char_name);
            match row.highest_complete() {
                Some(level) => {
                    print!("has completed level {} ", level);
                    if level as isize == furthest.level as isize {
                        furthest.char_names.push(row.char_name.clone());
                    }
                    if level as isize > furthest.level as isize {
                        furthest.level = level as i32;
                        furthest.char_names.clear();
                        furthest.char_names.push(row.char_name.clone());
                    }
                }
                None => (),
            }
            println!();
        }
    }
    match furthest.level {
        -1 => None,
        _ => Some(furthest),
    }
}

fn get_lowest_level(rows: &Vec<Row>) -> Option<PeopleAtLevel> {
    let mut slowest = PeopleAtLevel {
        char_names: Vec::new(),
        level: 10,
    };
    for row in rows {
        if !row.char_name.is_empty() {
            print!("Person: {} ", row.char_name);
            match row.highest_complete() {
                Some(level) => {
                    print!("has completed level {} ", level);
                    if level as isize == slowest.level as isize {
                        slowest.char_names.push(row.char_name.clone());
                    }
                    if (level as isize) < slowest.level as isize {
                        slowest.level = level as i32;
                        slowest.char_names.clear();
                        slowest.char_names.push(row.char_name.clone());
                    }
                }
                None => (),
            }
            println!();
        }
    }
    match slowest.level {
        10 => None,
        _ => Some(slowest),
    }
}

fn get_duty(rows: &Vec<Row>) -> Option<PeopleAtDuty> {
    let mut info = PeopleAtDuty {
        level: Vec::new(),
        duty: Vec::new(),
        char_names: Vec::new(),
    };
    for row in rows {
        if !row.char_name.is_empty() {
            print!("Duty: {} ", row.char_name);
            match row.highest_duty() {
                Some((duty, level)) => {
                    print!("is at a level {} {:?} ", level, String::from(&duty));
                    info.level.push(level as i32);
                    info.duty.push(duty.clone());
                    info.char_names.push(row.char_name.clone());
                }
                None => (),
            }
            println!();
        }
    }
    match info.char_names.len() {
        0 => None,
        _ => Some(info),
    }
}

fn get_prog(rows: &Vec<Row>) -> Option<PeopleAtDuty> {
    let mut info = PeopleAtDuty {
        level: Vec::new(),
        duty: Vec::new(),
        char_names: Vec::new(),
    };
    for row in rows {
        if !row.char_name.is_empty() {
            print!("Duty: {} ", row.char_name);
            match row.highest_prog() {
                Some((duty, level)) => {
                    print!("is level {} {} ", level, String::from(&duty));
                    info.level.push(level as i32);
                    info.duty.push(duty.clone());
                    info.char_names.push(row.char_name.clone());
                }
                None => (),
            }
            println!();
        }
    }
    match info.char_names.len() {
        0 => None,
        _ => Some(info),
    }
}

pub fn status_message() -> String {
    let rows = get_status_rows();
    let mut status = String::new();
    let mut completed_level: u32 = LEVEL_FLOOR - 1;
    let mut furthest_level: u32 = LEVEL_FLOOR - 1;
    match get_everyone_completed_level(&rows) {
        None => {}
        Some((level, people)) => {
            completed_level = level + LEVEL_FLOOR;
            if people == 0 {
                status = (status
                    + format!("Hmm, small hiccup with the tracker, kupo!\n\n").as_str())
                .to_string();
                completed_level = 999;
            } else if level == 10 {
                status = (status + format!("🎉 Everyone is done with the MSQ, kupo!\n\n").as_str())
                    .to_string();
            } else if people > 0 {
                status = (status + format!("📣 Everyone is done with level {} MSQ, so those spoilers are fine, kupo!\n\n", completed_level).as_str()).to_string();
            } else {
                status = (status
                    + format!("😢 I wasn't able to see anyone on the tracker, kupo...").as_str())
                .to_string();
            }
        }
    };
    match get_furthest_level(&rows) {
        None => {}
        Some(people) => {
            furthest_level = people.level as u32;
            status = (status
                + format!(
                    "🏃 Furthest along, completed level {} MSQ:\n",
                    people.level + LEVEL_FLOOR as i32
                )
                .as_str())
            .to_string();
            for p in people.char_names {
                status = (status + format!("\t• {}\n", p).as_str()).to_string();
            }
            status = (status + format!("\n").as_str()).to_string();
        }
    };
    match get_lowest_level(&rows) {
        None => {}
        Some(people) => {
            if furthest_level > people.level as u32 {
                status = (status
                    + format!(
                        "🐌 Taking their time, completed level {} MSQ:\n",
                        people.level + LEVEL_FLOOR as i32
                    )
                    .as_str())
                .to_string();
                for p in people.char_names {
                    status = (status + format!("\t• {}\n", p).as_str()).to_string();
                }
                status = (status + format!("\n").as_str()).to_string();
            }
        }
    };

    match get_duty(&rows) {
        None => {}
        Some(duty_info) => {
            status = (status
                + format!(
                    "⌛ {} Queuing for duties:\n",
                    duty_info.char_names.len() as i32
                )
                .as_str())
            .to_string();
            for (i, p) in duty_info.char_names.iter().enumerate() {
                let mut duty_msg = "";
                if matches!(duty_info.duty[i], MsqLevelStatus::WaitingOnDungeon) {
                    duty_msg = "dungeon"
                }
                if matches!(duty_info.duty[i], MsqLevelStatus::WaitingOnTrial) {
                    duty_msg = "trial"
                }
                status = (status
                    + format!(
                        "\t• {} is at a level {} {}\n",
                        p,
                        duty_info.level[i] + LEVEL_FLOOR as i32,
                        duty_msg
                    )
                    .as_str())
                .to_string();
            }
            status = (status + format!("\n").as_str()).to_string();
        }
    };

    match get_prog(&rows) {
        None => {}
        Some(duty_info) => {
            status = (status
                + format!(
                    "🚀 {} Progressing the MSQ:\n",
                    duty_info.char_names.len() as i32
                )
                .as_str())
            .to_string();
            for (i, p) in duty_info.char_names.iter().enumerate() {
                let mut duty_msg = "";
                if matches!(duty_info.duty[i], MsqLevelStatus::MSQInProgress) {
                    duty_msg = "MSQ"
                }
                status = (status
                    + format!(
                        "\t• {} is working on level {} {}\n",
                        p,
                        duty_info.level[i] + LEVEL_FLOOR as i32,
                        duty_msg
                    )
                    .as_str())
                .to_string();
            }
            status = (status + format!("\n").as_str()).to_string();
        }
    };

    if completed_level < LEVEL_FLOOR + 10 {
        status = (status + format!("🤐 Please make sure any MSQ spoilers level {} or higher are marked and hidden appropriately, kupo! 🤫\n\n", completed_level+1).as_str()).to_string();
    }

    status = (status
        + format!("🌕 Endwalker MSQ Tracker: https://inferiorlattice.com/ewtracker\n\n").as_str())
    .to_string();

    status.clone()
}

#[allow(unreachable_code)]
pub(crate) fn status_loop(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;
    let mut t0 = String::from("");
    loop {
        let now = status_message().clone();
        if t0.as_str().ne(now.as_str()) {
            bot.send_message(&status_message(), room, MessageType::TextMessage);
            t0 = status_message().clone();
        }
        println!("{}", status_message().as_str());
        thread::sleep(std::time::Duration::from_millis(120_000)); // sleep 120s
    }
    HandleResult::StopHandling
}

pub(crate) fn status(bot: &ActiveBot, message: &Message, cmd: &str) -> HandleResult {
    let room = &message.room;

    bot.send_message(&status_message(), room, MessageType::TextMessage);
    HandleResult::StopHandling
}
