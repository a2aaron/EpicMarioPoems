pub fn super_mario_epic(epic_title: &str) -> String {
    format!("Super Mario {}", epic_title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_super_mario_epic() {
        assert_eq!(super_mario_epic("Illiad"), "Super Mario Illiad");
        assert_eq!(super_mario_epic("Epic of Gilgamesh"), "Super Mario Epic of Gilgamesh");
        assert_eq!(super_mario_epic("Odyssey"), "Super Mario Odyssey");
    }
}
