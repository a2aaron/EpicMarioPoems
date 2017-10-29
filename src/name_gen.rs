pub fn generate_simple_name(epic_title: String) -> String {
    return "Super Mario ".to_string() + &epic_title;
}

#[cfg(test)]
mod tests {
    use name_gen::*;

    #[test]
    fn test_generate_simple_name() {
        assert_eq!(generate_simple_name("Illiad".to_string()), "Super Mario Illiad");
        assert_eq!(generate_simple_name("Epic of Gilgamesh".to_string()), "Super Mario Epic of Gilgamesh");
        assert_eq!(generate_simple_name("Odyssey".to_string()), "Super Mario Odyssey");
    }
}
