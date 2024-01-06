use rand::Rng;
use std::collections::HashMap;

pub struct Game {
    guess: String,
}

pub struct GameResult {
    pub result: String,
}

impl Game {
    pub fn new(args: &Vec<String>) -> Result<Game, &str> {
        if args.len() != 2 {
            return Err("You should provide your guess! Either rock, paper or scissors!");
        }

        let possible_guess: &str = &args[1].to_lowercase();

        Ok(Game {
            guess: possible_guess.to_string(),
        })
    }
}

pub fn run(game: Game) -> Result<GameResult, &'static str> {
    let possible_guesses = vec!["rock", "paper", "scissors"];

    if !possible_guesses.contains(&game.guess.as_str()) {
        return Err("You should check your guess spelling or chose your guess correctly !");
    }

    let mut rng = rand::thread_rng();
    let guesses_winners = HashMap::from([
        ("rock", "paper"),
        ("paper", "scissors"),
        ("scissors", "rock"),
    ]);
    let computer_guess: &str = possible_guesses[rng.gen_range(0..3)];

    println!(
        "Your guess is {}, computer guess is {}.",
        &game.guess, computer_guess
    );

    if guesses_winners.get(computer_guess) == Some(&game.guess.as_str()) {
        Ok(GameResult {
            result: "You won! 😄".to_string(),
        })
    } else if computer_guess == game.guess.as_str() {
        Ok(GameResult {
            result: "Draw! 🤨".to_string(),
        })
    } else {
        Ok(GameResult {
            result: "You lost! 🥲".to_string(),
        })
    }
}
