# monkeys
Monkeys is a 2D game coded fully in Rust for a school project.

The principle is very simple:
You play as a pirate that escaped a wrecked boat and landed on a island full of pirate hungry monkeys.
There are two types of monkeys:
- Erractic monkeys: they move in a random direction
- Hunter monkeys: they chase the pirate

Your goal is to find the island's hidden treasure while dodging the monkeys. If any of them comes across you, you lose.
To do so you have to move the pirate on island until you find it (for testing purposes, you can display the treasure in the 'island.rs' file).
To move, your pirate consumes energy, you can refill your energy using rhum bottles scattered across the island.
If you run out of energy, you lose.
When you find the treasure, you'll go to the next level, which will be harder.

Interface explained:
'P' represents your pirate, move it using the arrow keys
'B' represents the rhum bottles that refill your energy
'E' represents the Erratic monkeys
'H' represents the Hunter monkeys
'T' represents the treasure [DEBUG ONLY]

[Tips and tricks]
- When in a game, you can press 'Q' to leave the game
- When the game asks you to press any key to continue, you can press 'S' to directly start the game or 'Q' to directly quit it

[Known issues]
- When a monkey moves on a pirate, the player has to press a key to go back to the menu
