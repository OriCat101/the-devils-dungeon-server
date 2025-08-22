# The devils dungeon level server

This is the server that hosts the levels for the devils dungeon. Feel free to use to run your own server and/or submit pull requests.


## Setup to use
1. Config the .env / enter the nix-shell
2. Run the bin/setup.rs
3. Run the main.rs

## Docker
`docker compose up --build` to build it
Then run to migrate the db
`docker compose run --rm server ./setup`
Then run it like normal

## Development

### Adding a new migration
run `sqlx migrate add <name here>` to generate a new migration.


## Todo
 - Likes/Dislikes
 - Rating levels? (based on how hard they are)
 - Delete levels
 - Update levels
