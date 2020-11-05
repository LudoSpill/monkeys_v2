# monkeys
This is a 2D game coded fully in Rust as a school project.

The principle is very simple:
You play as a pirate that escaped a wrecked boat and landed on a island full of pirate hungry monkeys.
There are two types of monkeys:
- Erractic monkeys: they move in a random direction
- Hunter monkeys: they chase the pirate

Your goal is to find the island's hidden treasure.
To do so you have to move the pirate on island until you find it (for testing purposes, you can display the treasure in the 'island.rs' file).
To move, your pirate consumes energy, you can refill your energy using rhum bottles scattered across the island.
When you find the treasure, you'll go to the next level, which will be harder.

Interface explained:
'P' represents your pirate, move it using the arrow keys
'B' represents the rhum bottles that refill your energy
'E' represents the Erratic monkeys
'H' represents the Hunter monkeys
'T' represents the treasure [DEBUG ONLY]
