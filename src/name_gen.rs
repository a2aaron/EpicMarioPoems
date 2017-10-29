use rand::{self, Rng};
use epic;


pub fn random_msg() -> String {
    let epic = select_epic();
    epic_msg(epic)
}

pub fn super_mario_epic(epic_title: &str) -> String {
    format!("Super Mario {}", epic_title)
}

fn select_epic() -> &'static epic::Epic {
    rand::thread_rng().choose(epic::EPICS).unwrap()
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
