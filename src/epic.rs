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
    // 7th century
    Epic {
        title: "Táin Bó Cúailnge",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Bhaṭṭikāvya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kirātārjunīya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Shishupala Vadha",
        wiki: Wiki::Same,
    },
    // 8th to 10th century
    Epic {
        title: "Beowulf",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Waldere",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Daredevils of Sassoun",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Bhagavata Purana",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Hildebrandslied",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Muspilli",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kakawin Ramayana",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Shahnameh",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Waltharius",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Poetic Edda",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Vikramarjuna Vijaya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ādi purāṇa",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ajitha purana",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Gadaayuddha",
        wiki: Wiki::Name("Ranna#Literature"),
    },
    Epic {
        title: "Neelakesi",
        wiki: Wiki::Same,
    },
    // 11th century
    Epic {
        title: "Taghribat Bani Hilal",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ruodlieb",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Digenes Akritas",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of King Gesar",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Carmen Campidoctoris",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Borzu Nama",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Faramarz Nama",
        wiki: Wiki::Same,
    },
    // 12th century
    Epic {
        title: "Acallam na Senórach",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Song of Roland",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Knight in the Panther's Skin",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Alexandreis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "De bello Troiano",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Antiocheis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Carmen de Prodicione Guenonis",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Architrenius",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Liber ad honorem Augusti",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Tale of Igor's Campaign",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Bylina",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Naishadha Charita",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Roman de Troie",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Poem of Almería",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Roman de Brut",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Roman de Rou",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Eupolemius",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Bahman Nama",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kush Nama",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Banu Goshasp",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ramavataram",
        wiki: Wiki::Same,
    },
    // 13th century
    Epic {
        title: "Nibelungenlied",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kudrun",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Brut",
        wiki: Wiki::Name("Layamon's Brut"),
    },
    Epic {
        title: "Chanson de la Croisade Albigeoise",
        wiki: Wiki::Name("Song of the Albigensian Crusade"),
    },
    Epic {
        title: "Antar",
        wiki: Wiki::Name("Antarah ibn Shaddad"),
    },
    Epic {
        title: "Sirat al-Zahir Baibars",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Osman's Dream",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Cantar de Mio Cid",
        wiki: Wiki::Same,
    },
    Epic {
        title: "De triumphis ecclesiae",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Gesta Regum Britanniae",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Van den vos Reynaerde",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Poema de Fernán González",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Jewang Ungi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Basava purana",
        wiki: Wiki::Same,
    },
    // 14th century
    Epic {
        title: "Divine Comedy",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Cursor Mundi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Africa",
        wiki: Wiki::Name("Africa (Petrarch)"),
    },
    Epic {
        title: "The Tale of the Heike",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Brus",
        wiki: Wiki::Same,
    },
    Epic {
        title: "La Spagna",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Mocedades de Rodrigo",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Siege of Jerusalem",
        wiki: Wiki::Name("Siege of Jerusalem (poem)"),
    },
    Epic {
        title: "Zafarnamah",
        wiki: Wiki::Name("Zafarnamah (Mustawfi)"),
    },
    // 15th century
    Epic {
        title: "Yuan Phai",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Mahachat Kham luang",
        wiki: Wiki::Name("Thai_literature#Maha-chat_Kham_Luang:_the_.22Great_Birth.22_Sermon"),
    },
    Epic {
        title: "Orlando innamorato",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Shmuel-Bukh",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Mlokhim-Bukh",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Book of Dede Korkut",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Morgante",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Wallace",
        wiki: Wiki::Name("The Wallace (poem)"),
    },
    Epic {
        title: "Troy Book",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Heldenbuch",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ibong Adarna",
        wiki: Wiki::Same,
    },
    // 16th century
    Epic {
        title: "Lilit Phra Lo",
        wiki: Wiki::None,
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
    // 20th century epics
    Epic {
        title: "The Divine Enchantment",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Lahuta e Malcís",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ural-batyr",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Ballad of the White Horse",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Mensagem",
        wiki: Wiki::Name("Fernando Pessoa#Mensagem"),
    },
    Epic {
        title: "The Cantos",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Dorvyzhy",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Legend of Sigurd and Gudrún",
        wiki: Wiki::Same,
    },
    Epic {
        title: "A Cycle of the West",
        wiki: Wiki::Name("Cycle of the West"),
    },
    Epic {
        title: "The Odyssey: A Modern Sequel",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Dymer",
        wiki: Wiki::Same,
    },
    Epic {
        title: "\"A\"",
        wiki: Wiki::Name("Louis_Zukofsky#.22A.22"),
    },
    Epic {
        title: "John Brown's Body",
        wiki: Wiki::Name("John Brown's Body (poem)"),
    },
    Epic {
        title: "The Fall of Arthur",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Bridge",
        wiki: Wiki::Name("The Bridge (long poem)"),
    },
    Epic {
        title: "Kamayani",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Canto General",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Paterson",
        wiki: Wiki::Name("Paterson (poem)"),
    },
    Epic {
        title: "Sugata Saurabha",
        wiki: Wiki::Name("Sugata Saurabha (epic)"),
    },
    Epic {
        title: "Victory for the Slain",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Rashmirathi",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Hunkar",
        wiki: Wiki::Name("Hunkar (epic poem)"),
    },
    Epic {
        title: "Savitri",
        wiki: Wiki::Name("Savitri: A Legend and a Symbol"),
    },
    Epic {
        title: "The Maximus Poems",
        wiki: Wiki::Name("Charles Olson#The Maximus Poems"),
    },
    Epic {
        title: "Aniara",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Song of Lawino",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Banner of Joan",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kristubhagavatam",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Changing Light at Sandover",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Battlefield Where The Moon Says I Love You",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Emperor Shaka the Great",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Lay of the Children of Húrin",
        wiki: Wiki::Same,
    },
    Epic {
        title: "The Lay of Leithian",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Giannina Braschi's Empire of Dreams",
        wiki: Wiki::Name("Braschi's Empire of Dreams"),
    },
    Epic {
        title: "Omeros",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Arundhati",
        wiki: Wiki::Name("Arundhati (epic)"),
    },
    Epic {
        title: "Mastorava",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Astronautilía Hvězdoplavba",
        wiki: Wiki::Name("Astronautilia"),
    },
    Epic {
        title: "Fredy Neptune: A Novel in Verse",
        wiki: Wiki::Name("Fredy Neptune"),
    },
    // 21st century
    Epic {
        title: "Sribhargavaraghaviyam",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Ashtavakra",
        wiki: Wiki::Name("Ashtavakra (epic)"),
    },
    Epic {
        title: "Gitaramayanam",
        wiki: Wiki::Same,
    },
    // Other epics
    Epic {
        title: "Gesta Berengarii imperatoris",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of Bamana Segu",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of Darkness",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of Jangar",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of Köroğlu",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of the Forgotten",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Hikayat Seri Rama",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Hinilawod",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Klei Khan Y Dam San",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Koti and Chennayya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Epic of Siri",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Kutune Shirka",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Parsifal",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Pasyón",
        wiki: Wiki::Name("Pasyon"),
    },
    Epic {
        title: "Ramakien",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Der Ring des Nibelungen",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Siribhoovalaya",
        wiki: Wiki::Same,
    },
    Epic {
        title: "Sundiata",
        wiki: Wiki::Name("Epic of Sundiata"),
    },
    Epic {
        title: "Yama Zatdaw",
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
