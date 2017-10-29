pub fn generate_simple_name(epic_title: &str) -> String {
    return format!("Super Mario {}", epic_title)
}

#[cfg(test)]
mod tests {
    use name_gen::*;

    #[test]
    fn test_generate_simple_name() {
        assert_eq!(generate_simple_name("Illiad"), "Super Mario Illiad");
        assert_eq!(generate_simple_name("Epic of Gilgamesh"), "Super Mario Epic of Gilgamesh");
        assert_eq!(generate_simple_name("Odyssey"), "Super Mario Odyssey");
    }
}
