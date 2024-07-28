# Rust SDL2 Pong
This is a simple implementation of the classic Pong game using Rust and the SDL2 library. This project is being done to test Copilot.

## Prerequisites

Before running the game, make sure you have the following installed:

- Rust (version 1.75.0)
- SDL2 (version 2.0.20)

## About

This project is a simple implementation of the classic Pong game using Rust and the SDL2 library. It serves as a test for GitHub Copilot's capabilities in assisting with software development tasks. (The topic was proposed by Copilot as well).

### Objective:
Develop a simple Pong game using Rust and SDL2. The game will feature two paddles and a ball, with basic collision detection and scoring.

### Requirements:
1. **Paddles:**
   - Create two paddles that can be controlled using the keyboard (W/S for the left paddle, Up/Down arrows for the right paddle).
   - Implement smooth movement for the paddles.

2. **Ball:**
   - Create a ball that moves across the screen.
   - Implement collision detection between the ball and the paddles.
   - Implement collision detection between the ball and the top/bottom edges of the screen.

3. **Scoring System:**
   - Implement a scoring system that increases the score when the ball passes a paddle.
   - Display the score on the screen.

4. **Game Loop:**
   - Implement a game loop that updates the game state and renders the game at a consistent frame rate.
   - Handle game events such as player input and window close.

### Steps:

1. **Set Up Your Project:**
   - Create a new Rust project using Cargo.
   - Add the `sdl2` crate to your `Cargo.toml`.

2. **Initialize SDL2:**
   - Initialize the SDL2 library.
   - Create a window and a canvas for rendering.

3. **Paddles:**
   - Define a struct for the paddles.
   - Implement movement logic for the paddles based on keyboard input.

4. **Ball:**
   - Define a struct for the ball.
   - Implement logic to move the ball and detect collisions with the paddles and screen edges.

5. **Scoring System:**
   - Implement a scoring system that increases the score when the ball passes a paddle.
   - Render the score on the screen.

6. **Game Loop:**
   - Implement the main game loop to update the game state and render the game.
   - Handle events such as player input and window close.

### Pseudocode:

1. Create a new Rust project.
2. Add `sdl2` to `Cargo.toml`.
3. Initialize SDL2.
4. Create a window and canvas.
5. Define structs for the paddles and ball.
6. Implement movement logic for the paddles.
7. Implement logic to move the ball and detect collisions.
8. Implement a scoring system.
9. Implement the main game loop.

This exercise will help you practice integrating multiple aspects of game development, including rendering, input handling, collision detection, and game state management using Rust and SDL2.
