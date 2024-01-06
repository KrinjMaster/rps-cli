use rps_cli::{run, Game};
use std::env;
use std::process;

fn main() {
    let input: Vec<String> = env::args().collect();

    let game = Game::new(&input).unwrap_or_else(|err: &str| {
        eprintln!("{}", err);
        process::exit(1);
    });

    match run(game) {
        Ok(output) => println!("{}", output.result),
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_args_length() {
        let input: Vec<String> = vec!["/route/to/dir/".to_string()];

        assert_eq!(Game::new(&input).is_err(), true);
    }
    #[test]
    fn test_if_args_correct() {
        let input: Vec<String> = vec!["/route/to/dir/".to_string(), "i dunno".to_string()];

        let game = Game::new(&input).unwrap_or_else(|err: &str| {
            eprintln!("{}", err);
            process::exit(1);
        });

        assert_eq!(run(game).is_err(), true);
    }
}
