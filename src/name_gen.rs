use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Write};

use rand::thread_rng;
use rand::distributions::{Weighted, WeightedChoice, Sample};
use epic;
use fs2::FileExt;


/// Generates a random message, selecting an epic and a message format.
pub fn random_msg() -> String {
    let epic = select_epic();
    epic_msg(epic)
}

pub fn super_mario_epic(epic_title: &str) -> String {
    format!("Super Mario {}", epic_title)
}

/// This selects an epic, preferring ones that haven't been chosen too recently.
/// It works by using a file "epics.log" listed in order as they're read. It
/// then uses the order to prioritize the random selection, and writes its
/// choice back into the file.
fn select_epic() -> &'static epic::Epic {
    let num_skip = 5;
    let num_ease = 10;
    let num_weight = num_skip + num_ease;

    // @Robustness: Handle these unwraps better?
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("epics.log")
        .unwrap();
    file.lock_exclusive().unwrap();

    // Make the weights, set 0 for the 5 most recent...
    let mut weights = HashMap::new();
    let mut file = BufReader::new(file);
    let mut lines = Vec::new();
    let mut line = String::new();
    while let Ok(x) = file.read_line(&mut line) {
        if x == 0 {
            break;
        }
        lines.push(String::from(line.trim()));
        line.clear();
    }

    for line in lines.iter().rev().take(num_skip) {
        weights.entry(line.clone()).or_insert(0);
    }
    // Then doubling weight for each of the next 10 items
    let mut weight = 1;
    for line in lines.iter().rev().skip(num_skip).take(num_ease) {
        weights.entry(line.clone()).or_insert(weight);
        weight *= 2;
    }
    // Any item that remains gets a weight of 1024

    let mut epics: Vec<_> = epic::EPICS
        .iter()
        .map(|epic| {
            Weighted {
                weight: *weights.get(epic.title).unwrap_or(&weight),
                item: epic,
            }
        })
        .collect();
    let choice = WeightedChoice::new(&mut epics[..]).sample(&mut thread_rng());

    // We record which one we chose so it doesn't get chosen too soon.
    lines.push(choice.title.into());
    let mut file = file.into_inner();
    // @Robustness: Handle these unwraps?
    // We only write the weighted entries so that it doesn't grow forever
    file.set_len(0).unwrap();
    for line in &lines[lines.len().saturating_sub(num_weight + 1)..] {
        writeln!(file, "{}", line).unwrap();
    }
    file.unlock().unwrap();
    choice
}

fn epic_msg(epic: &epic::Epic) -> String {
    match epic.wiki() {
        Some(wiki) => {
            let title = super_mario_epic(epic.title);
            format!("{}\n\n({})", title, wiki)
        }
        None => {
            format!(
                "{}",
                super_mario_epic(epic.title),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_mario_epic() {
        assert_eq!(super_mario_epic("Illiad"), "Super Mario Illiad");
        assert_eq!(
            super_mario_epic("Epic of Gilgamesh"),
            "Super Mario Epic of Gilgamesh"
        );
        assert_eq!(super_mario_epic("Odyssey"), "Super Mario Odyssey");
    }
}
