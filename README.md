# Guess the Number Game

The "Guess the Number" game is a classic and straightforward project that can help you get started with Rust programming. In this game, the computer randomly selects a number within a specified range, and the player needs to guess that number.

Here's a step-by-step outline of how you can implement this project:

1. Generate a random number: Use the `rand` crate to generate a random number between a chosen range (e.g., 1 to 100).

2. Input from the player: Take input from the player to guess the number. You can use the `std::io` module to read the player's input from the console.

3. Compare the guess: Compare the player's guess with the randomly generated number.

4. Provide feedback: Based on the player's guess, provide feedback if the guess is too high or too low.

5. Loop until correct: Keep prompting the player for guesses until they guess the correct number.

6. Track attempts: Keep track of the number of attempts the player made before guessing the correct number.

7. Display the result: Display a message when the player guesses the number correctly, along with the number of attempts taken.

8. Allow replay: Ask the player if they want to play again and repeat the game accordingly.
