use std::{fs::File, io::BufReader, io::BufWriter, io::Write, error::Error};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Game {
    name: String,
    year: u16,
    platforms: Vec<String>,
    developer: String,
}

struct GameIterator<'a> {
    games: &'a[Game],
    idx: usize,
}

impl<'a> GameIterator<'a> {
    fn new(games: &'a[Game]) -> Self {
        GameIterator {games, idx: 0}
    }
}

impl<'a> Iterator for GameIterator<'a> {
    type Item = &'a Game;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.games.len() {
            let game = self.games.get(self.idx);
            self.idx += 1;
            return game;
        }
        None
    }
}

fn read_games_from_json_file(json_path: &str) -> Result<Vec<Game>, Box<dyn Error>> {
    let json_file = File::open(json_path)?;
    let reader = BufReader::new(json_file);
    let games: Vec<Game> = serde_json::from_reader(reader)?;
    Ok(games)
}

fn write_games_to_yaml_file(games: &[Game], yaml_path: &str) -> Result<(), Box<dyn Error>> {
    let yaml_str = serde_yaml::to_string(&games)?;
    let yaml_file = File::create(yaml_path)?;
    let mut writer = BufWriter::new(yaml_file);
    writer.write_all(yaml_str.as_bytes())?;
    Ok(())
}

fn check_out_game_iterator(games: &[Game]) {
    let iter = GameIterator::new(games);
    for game in iter {
        println!("{}", game.name);
    }
}

fn main() {
    let games = read_games_from_json_file("data/games.json").unwrap();
    // let _ = write_games_to_yaml_file(&games, "data/games.yaml");
    check_out_game_iterator(&games);
}
