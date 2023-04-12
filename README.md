# An Adventure Game usin OpenAIs API

This is a simple Rust project that demonstrates the use of the OpenAIs API to create an adventure game that a user can play by typing text into the console. The game master will guide the user through a map.

## Setup
To use this project, you will need an OpenAI API key. You can sign up for an API key [here](https://beta.openai.com/signup/).

Once you have an API key, you will need to create a configuration file at `~/.adventure.ini` with the following contents:

```ini
OPENAI_API_KEY = <your-api-key>
```

Replace `<your-api-key>` with your actual API key.

## Running the Game

To run the game, simply clone the repository, navigate to the project directory, and run the following command:

```sh
cargo run
```

The game will start and prompt you to enter text into the console. The game master will respond to your input with descriptions of the game world and guidance on what to do next.

## License
This project is licensed under the [MIT License](LICENSE).

