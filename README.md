# Build a 🐍 Snake game with Copilot

This is an example repository that has a Rust API with everything you need for building an interactive Snake Game.

## Scenario

You've been just hired and an engineer who is no longer around has left a Rust project that is the backbone for a Snake Game. You must develop a web application to make it interactive, then package it as a container, and set some automation.

Copilot to the rescue!

## API

The API is a Rust application that runs on `127.0.0.1` using port `8080``. It provides a JSON response with the following format:

```json
    {"body": [[x1, y1], [x2, y2], ...], "food": [xf, yf], "game_over": true/false}
```

Here are a couple of example responses from the API:

```json
    {"body": [[0, 0], [0, 1], [0, 2]], "food": [1, 2], "game_over": false}
    {"body": [[0, 0], [0, 1], [0, 2]], "food": [1, 2], "game_over": true}
```

## TL;DR ?

If you want to just get it done, you can use the following prompt to Copilot Chat to accomplish this:

    Create an HTML file that uses a Rust application running on port 8080 to create a Snake game. The Rust application should provide a JSON response with the following format: {"body": [[x1, y1], [x2, y2], ...], "food": [xf, yf], "game_over": true/false}. The game should have a canvas element for the game board, and should display the current score. The game should start when the HTML file is loaded, and should allow the player to control the snake using the arrow keys. The Rust application should handle the game logic and provide the necessary data to update the game board and score display in the HTML file. The game should end when the game_over field in the JSON response is true, and should display a message to the player indicating that the game is over. The player should be able to restart the game by pressing a button.
