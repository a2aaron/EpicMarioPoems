mod name_gen;
mod epic;

fn main() {
    for epic in epic::EPICS {
        match epic.wiki() {
            Some(wiki) => {
                let title = name_gen::super_mario_epic(epic.title);
                println!("{} ({})", title, wiki);
            }
            None => {
                println!(
                    "{}",
                    name_gen::super_mario_epic(epic.title),
                )
            }
        }
    }
}
