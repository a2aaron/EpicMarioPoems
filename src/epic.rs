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
    // 18th century
    Epic {
        title: "Kumulipo",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Henriade",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Utendi wa Tambuka",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Pucelle d'Orléans",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Seasons",
        wiki: Wiki::Name("The Seasons (poem)"),
    },
    Epic {
        title: "O Uraguai",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Caoineadh Airt Uí Laoghaire",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Caramuru",
        wiki: Wiki::Name("Caramuru (epic poem)"),
    },
    Epic {
        title: "Joan of Arc",
        wiki: Wiki::Name("Joan of Arc (poem)"),
    },
    Epic {
        title: "Hermann and Dorothea",
        wiki: Wiki::Same,
    },
    // 19th century
    Epic {
        title: "The Tale of Kiều",
        wiki: Wiki::Name("The Tale of Kieu"),
    },
    Epic {
        title: "Thalaba the Destroyer",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Madoc",
        wiki: Wiki::Name("Madoc (poem)"),
    },
    Epic {
        title: "The Columbiad",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Milton: A Poem",
        wiki: Wiki::Name("Milton: A Poem in Two Books"),
    },
    Epic {
        title: "Marmion",
        wiki: Wiki::Name("Marmion (poem)"),
    },
    Epic {
        title: "Childe Harold's Pilgrimage",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Queen Mab",
        wiki: Wiki::Name("Queen Mab (poem)"),
    },
    Epic {
        title: "Roderick the Last of the Goths",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Lord of the Isles",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Alastor, or The Spirit of Solitude",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Revolt of Islam",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Harold the Dauntless",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Endymion",
        wiki: Wiki::Name("Endymion (poem)"),
    },
    Epic {
        title: "Hyperion",
        wiki: Wiki::Name("Hyperion (poem)"),
    },
    Epic {
        title: "The Fall of Hyperion",
        wiki: Wiki::Name("The Fall of Hyperion: A Dream"),
    },
    Epic {
        title: "The Battle of Marathon",
        wiki: Wiki::Name("The Battle of Marathon: A Poem"),
    },
    Epic {
        title: "Phra Aphai Mani",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Don Juan",
        wiki: Wiki::Name("Don Juan (Byron)"),
    },
    Epic {
        title: "Camões",
        wiki: Wiki::None,
    },
    Epic {
        title: "Dona Branca",
        wiki: Wiki::None,
    },
    Epic {
        title: "Tamerlane",
        wiki: Wiki::Name("Tamerlane (poem)"),
    },
    Epic {
        title: "Creation, Man and the Messiah",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Prometheus Bound",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Pan Tadeusz",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Baptism on the Savica",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Florante at Laura",
        wiki: Wiki::Same,
    },
    Epic {
        title: "King Alfred",
        wiki: Wiki::Name("King Alfred (poem)"),
    },
    Epic {
        title: "János Vitéz",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Smrt Smail-age Čengića",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Toldi",
        wiki: Wiki::None,
    },
    Epic {
        title: "Toldi szerelme",
        wiki: Wiki::None,
    },
    Epic {
        title: "Toldi estéje",
        wiki: Wiki::None,
    },
    Epic {
        title: "Toldi trilogy",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Evangeline",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Mountain Wreath",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Tales of Ensign Stål",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kalevala",
        wiki: Wiki::Same,
    },
    Epic {
        title: "I-Juca-Pirama",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kalevipoeg",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Prelude",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Song of Myself",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Song of Hiawatha",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Saga of King Olaf",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Aurora Leigh",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Meghnad Badh Kavya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Terje Vigen",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Légende des Siècles",
        wiki: Wiki::Name("La Légende des siècles"),
    },
    Epic {
        title: "The Earthly Paradise",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ibonia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Martín Fierro",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Idylls of the King",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Clarel",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Story of Sigurd the Volsung and the Fall of the Niblungs",
        wiki: Wiki::Same,
    },
    Epic {
        title: "L'Atlàntida",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Light of Asia",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The City of Dreadful Night",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Tristram of Lyonesse",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Eros and Psyche",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Fin de Satan",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Canigó",
        wiki: Wiki::Name("Canigó#Literature"),
    },
    Epic {
        title: "Lāčplēsis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Tabaré",
        wiki: Wiki::Name("Tabaré (poem)"),
    },
    Epic {
        title: "The Wanderings of Oisin",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kotan Utunnai",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Lục Vân Tiên",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Amir Arsalan",
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
