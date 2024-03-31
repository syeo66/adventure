# Adventure Game

This Rust project showcases the utilization of OpenAI's API to construct an adventure game playable via text input in the console. The game master directs the player through a virtual map.

## Setup
To utilize this project, you'll require an API key from either OpenAI or Anthropic.

1. **Obtain an API Key**: Acquire an API key from [OpenAI](https://openai.com) or [Anthropic](https://anthropic.com).

2. **Configuration**: After obtaining your API key, create a configuration file at `~/.adventure.ini` with the following structure:

    ```ini
    OPENAI_API_KEY=<your-api-key>
    ```

    or

    ```ini
    ANTHROPIC_API_KEY=<your-api-key>
    ```

    Replace `<your-api-key>` with your actual API key.

## Running the Game

To start the game, follow these steps:

1. **Clone the Repository**: Clone this repository to your local machine.

2. **Navigate to the Project Directory**: Open your terminal and navigate to the project directory.

3. **Run the Game**: Execute the following command in your terminal:

    ```sh
    cargo run
    ```

    This command initiates the game, prompting you to enter text into the console. The game master will respond to your input with descriptions of the game world and instructions on what to do next.

## License
This project is licensed under the [MIT License](LICENSE).
