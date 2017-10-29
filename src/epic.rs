/// An Epic object contains all the information about an epic poem that we care
/// about when we're generating messages.
#[derive(Debug)]
pub struct Epic {
    /// The title of the epic poem
    pub title: &'static str,
    /// We store the Wikipedia article that we want to use for attribution. If a
    /// given poem doesn't have an appropriate Wikipedia article to link to,
    /// this will contain `None`.
    wiki: Wiki,
}

impl Epic {
    /// Returns the URL of the Wikipedia article that we want to use for
    /// attribution. If a given poem doesn't have an appropriate Wikipedia
    /// article to link to, this will be `None`.
    pub fn wiki(&self) -> Option<String> {
        let name = match self.wiki {
            Wiki::None => return None,
            Wiki::Same => self.title,
            Wiki::Name(name) => name,
        };
        let name = name.replace(" ", "_");
        Some(format!("https://en.wikipedia.org/wiki/{}", name))
    }
}

#[derive(Debug, PartialEq)]
enum Wiki {
    None,
    Same,
    Name(&'static str),
}

/// A list of epic poems
pub static EPICS: &[Epic] = &[
    // 8th to 6th century BC
    Epic {
        title: "Epic of Gilgamesh",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Atrahasis",
        wiki: Wiki::Name("Atra-Hasis"),
    },
    Epic {
        title: "Enûma Eliš",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Legend of Keret",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Cycle of Kumarbi",
        wiki: Wiki::Same,
    },
    // 20th to 10th century BC
    Epic {
        title: "Iliad",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Odyssey",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Works and Days",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Theogony",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Shield of Heracles",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Catalogue of Women",
        wiki: Wiki::Same,
    },
];


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_wiki_url_same() {
        assert_eq!(
            Epic {
                title: "Odyssey",
                wiki: Wiki::Same,
            }.wiki(),
            Some("https://en.wikipedia.org/wiki/Odyssey".into())
        );
    }

    #[test]
    fn test_wiki_url_different() {
        assert_eq!(
            Epic {
                title: "Atrahasis",
                wiki: Wiki::Name("Atra-Hasis"),
            }.wiki(),
            Some("https://en.wikipedia.org/wiki/Atra-Hasis".into())
        );
    }

    #[test]
    fn test_wiki_url_none() {
        assert_eq!(
            Epic {
                title: "Dona Branca",
                wiki: Wiki::None,
            }.wiki(),
            None
        );
    }

    #[test]
    fn test_wiki_url_spaces() {
        assert_eq!(
            Epic {
                title: "Epic of Gilgamesh",
                wiki: Wiki::Same,
            }.wiki(),
            Some("https://en.wikipedia.org/wiki/Epic_of_Gilgamesh".into())
        );
    }
}
