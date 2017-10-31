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
    // 20th to 10th century BC
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
        wiki: Wiki::Name("Kumarbi"),
    },
    // 8th to 6th century BC
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
    Epic {
        title: "Cypria",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Aethiopis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Little Iliad",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Iliupersis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Nostoi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Telegony",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic Cycle",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Oedipodea",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Thebaid",
        wiki: Wiki::Name("Thebaid (Greek poem)"),
    },
    Epic {
        title: "Epigoni",
        wiki: Wiki::Name("Epigoni (epic)"),
    },
    Epic {
        title: "Alcmeonis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Theban Cycle",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Aegimius",
        wiki: Wiki::Name("Aegimius (poem)"),
    },
    Epic {
        title: "Astronomia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Descent of Perithous",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Idaean Dactyls",
        wiki: Wiki::Name("Idaean Dactyls (poem)"),
    },
    Epic {
        title: "Megala Erga",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Megalai Ehoiai",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Melampodia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Wedding of Ceyx",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Capture of Oechalia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Phocais",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Titanomachy",
        wiki: Wiki::Name("Titanomachy (epic poem)"),
    },
    Epic {
        title: "Danais",
        wiki: Wiki::Name("Danais (epic)"),
    },
    Epic {
        title: "Minyas",
        wiki: Wiki::Name("Minyas (poem)"),
    },
    Epic {
        title: "Naupactia",
        wiki: Wiki::Same,
    },
    // 8th century BC to 3rd century AD
    Epic {
        title: "Mahābhārata",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ramayana",
        wiki: Wiki::Same,
    },
    // 3rd century BC
    Epic {
        title: "Argonautica",
        wiki: Wiki::Same,
    },
    // 2nd century BC
    Epic {
        title: "Annales",
        wiki: Wiki::Name("Annales (Ennius)"),
    },
    // 1st century BC
    Epic {
        title: "De rerum natura",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Georgics",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Aeneid",
        wiki: Wiki::Same,
    },
    // 1st century AD
    Epic {
        title: "Argonautica",
        wiki: Wiki::Name("Gaius Valerius Flaccus#Writings"),
    },
    Epic {
        title: "Metamorphoses",
        wiki: Wiki::Name("Metamorphoses (poem)"),
    },
    Epic {
        title: "Pharsalia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Punica",
        wiki: Wiki::Name("Punica (poem)"),
    },
    Epic {
        title: "Thebaid",
        wiki: Wiki::Name("Thebaid (Latin poem)"),
    },
    Epic {
        title: "Achilleid",
        wiki: Wiki::Same,
    },
    // 2nd century
    Epic {
        title: "Buddhacarita",
        wiki: Wiki::Same,
    },
    // 2nd to 5th century
    Epic {
        title: "Cilappatikāram",
        wiki: Wiki::Name("Silappatikaram"),
    },
    Epic {
        title: "Manimekalai",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Cīvaka Cintāmaṇi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Valayapathi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kundalakēci",
        wiki: Wiki::Name("Kundalakesi"),
    },
    // 3rd to 4th century
    Epic {
        title: "Posthomerica",
        wiki: Wiki::Same,
    },
    Epic {
        title: "De raptu Proserpinae",
        wiki: Wiki::Name("Claudian#As_poet"),
    },
    // 4th century
    Epic {
        title: "Kumārasambhava",
        wiki: Wiki::Name("Kumaarasambhavam"),
    },
    Epic {
        title: "Raghuvaṃśa",
        wiki: Wiki::Same,
    },
    // 5th century
    Epic {
        title: "Argonautica Orphica",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Dionysiaca",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Mahavamsa",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Yadegar-e Zariran",
        wiki: Wiki::Same,
    },
    // 16th century
    Epic {
        title: "Lilit Phra Lo",
        wiki: Wiki::Name("Phra Lo"),
    },
    Epic {
        title: "Judita",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Orlando Furioso",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Davidiad",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Christiad",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Os Lusíadas",
        wiki: Wiki::Same,
    },
    Epic {
        title: "L'Amadigi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Araucana",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Gerusalemme liberata",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ramacharitamanasa",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Journey to the West",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Faerie Queene",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Venus and Adonis",
        wiki: Wiki::Name("Venus and Adonis (Shakespeare poem)"),
    },
    Epic {
        title: "Lucrece",
        wiki: Wiki::Name("The Rape of Lucrece"),
    },
    // 17th century
    Epic {
        title: "La Argentina",
        wiki: Wiki::Name("La Argentina (poem)"),
    },
    Epic {
        title: "La Cleopatra",
        wiki: Wiki::Name("La Cleopatra (poem)"),
    },
    Epic {
        title: "Biag ni Lam-ang",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Il Conquisto di Granata",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Exact Epitome of the Four Monarchies",
        wiki: Wiki::Name("The Tenth Muse Lately Sprung Up in America"),
    },
    Epic {
        title: "Szigeti veszedelem",
        wiki: Wiki::Name("Peril of Sziget"),
    },
    Epic {
        title: "Gondibert",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Paradise Lost",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Paradise Regained",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Khun Chang Khun Phaen",
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
