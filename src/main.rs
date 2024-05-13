use clap::{Parser, ValueEnum};
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
struct AnthropicContent {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnthropicCompletion {
    content: Vec<AnthropicContent>,
    model: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GptCompletion {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Model {
    Gpt3,
    Gpt4,
    Claude3,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Select the model, 'gpt3', 'gpt4' or 'claude3'
    #[arg(value_enum, default_value_t = Model::Claude3)]
    model: Model,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let model = match args.model {
        Model::Gpt3 => "gpt-3.5-turbo",
        Model::Gpt4 => "gpt-4o",
        Model::Claude3 => "claude-3-sonnet-20240229",
    };

    let stdin = std::io::stdin();

    let mut messages = vec![
        Message {
            role: "user".to_string(),
            content: "You are a game master in a fantasy role play game like Dungeons and Dragons. You will guide the player through a map with room descriptions and their connections to other rooms. Let the adventurer inspect at least five rooms during the adventure but only present them as the the adventure progresses. Some items can be taken and used in other places. You will have to guide the player. Never let the player know anything he did not yet discover and don't write anything a user should have answered. Write 'THE END' when the game ended because the player died, exits the game or won. Really just write 'THE END' in any case the game has ended. Don't forget the THE END. Start with a description of what the goal of the adventure is. Hello game master. I am ready. Let's start.".to_string(),
        },
    ];

    loop {
        let mut buffer = String::new();

        let body: RequestBody = RequestBody {
            model: model.to_string(),
            messages: messages.clone(),
            max_tokens: 500,
        };

        let response = if args.model == Model::Claude3 {
            get_anthrophic_response(body)?
        } else {
            get_openai_response(body)?
        };

        messages.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        println!(
            "-----------------------------------\n{}: {}\n\n",
            "Game Master", response
        );

        if response.contains("THE END") {
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

fn get_anthrophic_response(body: RequestBody) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.anthropic.com/v1/messages";
    let headers = build_headers(Model::Claude3)?;
    let client = reqwest::blocking::Client::new();
    let response: AnthropicCompletion = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()?
        .json()?;

    let msg = &response.content[0].text;

    Ok(msg.into())
}

fn get_openai_response(body: RequestBody) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.openai.com/v1/chat/completions";
    let headers = build_headers(Model::Gpt3)?;
    let client = reqwest::blocking::Client::new();
    let response: GptCompletion = client
        .post(url)
        .headers(headers)
        .json(&body)
        .send()?
        .json()?;

    let msg = &response.choices[0].message.content;

    Ok(msg.into())
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

    let key = settings.get_string("OPENAI_API_KEY").unwrap();

    if key.is_empty() {
        panic!("OPENAI_API_KEY not set");
    }

    key
}

fn get_anthropic_api_key() -> String {
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

    let key = settings.get_string("ANTHROPIC_API_KEY").unwrap();

    if key.is_empty() {
        panic!("ANTHROPIC_API_KEY not set");
    }

    key
}

fn build_headers(model: Model) -> Result<HeaderMap, Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    if model == Model::Claude3 {
        let api_key = get_anthropic_api_key();
        headers.insert("x-api-key", HeaderValue::from_str(&format!("{}", api_key))?);
        headers.insert("content-type", HeaderValue::from_str("application/json")?);
        headers.insert("anthropic-version", HeaderValue::from_str("2023-06-01")?);
    } else {
        let api_key = get_openai_api_key();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        );
    }
    Ok(headers)
}
