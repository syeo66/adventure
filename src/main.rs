use clap::Parser;
use config::{Config, File};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Choice {
    index: i64,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Completion {
    id: String,
    object: String,
    created: i64,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: i64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Use gpt-4 instead of gpt-3.5-turbo (will be more expensive and slower, but might lead to a better stories)
    #[arg(long)]
    gpt4: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let openai_api_key = get_openai_api_key();

    let api_key = &openai_api_key;
    let prompt = "Hello game master. I am ready. Let's start.";
    let model = if args.gpt4 { "gpt-4" } else { "gpt-3.5-turbo" };
    let url = "https://api.openai.com/v1/chat/completions";

    let stdin = std::io::stdin();

    let mut messages = vec![
        Message {
            role: "user".to_string(),
            content: "You are a game master in a fantasy role play game like Dungeons and Dragons. You will guide the player through a map with room descriptions and their connections to other rooms. Some items can be taken and used in other places. You will have to guide the player. Never let the player know anything he did not yet discover and don't write anything a user should have answered. Write 'THE END' when the game ended because the player died, exits the game or won. Really just write 'THE END' in any case the game has ended. Don't forget the THE END. Start with a description of what the goal of the adventure is.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        },
    ];

    loop {
        let mut buffer = String::new();

        // TODO let the API summarize the conversation to reduce the amount of
        // data in the request after a certain amount of tokens used

        let headers = build_headers(api_key)?;
        let body: RequestBody = RequestBody {
            model: model.to_string(),
            messages: messages.clone(),
            max_tokens: 200,
        };

        let client = reqwest::blocking::Client::new();
        let response: Completion = client
            .post(url)
            .headers(headers)
            .json(&body)
            .send()?
            .json()?;

        messages.push(Message {
            role: "assistant".to_string(),
            content: response.choices[0].message.content.clone(),
        });

        println!(
            "-----------------------------------\n{}: {}\n\n",
            "Game Master", response.choices[0].message.content
        );

        if response.choices[0].message.content.contains("THE END") {
            break Ok(());
        }

        println!("{}: ", "Your input");
        stdin.read_line(&mut buffer)?;

        messages.push(Message {
            role: "user".to_string(),
            content: buffer.clone(),
        });
    }
}

fn get_openai_api_key() -> String {
    let home_dir = match env::var("HOME") {
        Ok(path) => path,
        Err(_) => panic!("Unable to get home directory path"),
    };
    let file_path = PathBuf::from(format!("{}/.adventure.ini", home_dir));

    let settings = match Config::builder()
        .add_source(File::from(file_path.clone()))
        .build()
    {
        Ok(config) => config,
        Err(_) => panic!("Unable to load config file at {:?}", file_path),
    };

    settings.get_string("OPENAI_API_KEY").unwrap()
}

fn build_headers(api_key: &str) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );
    Ok(headers)
}
