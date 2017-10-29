/// An Epic object contains all the information about an epic poem that we care
/// about when we're generating messages.
#[derive(Debug)]
pub struct Epic {
    /// The title of the epic poem
    pub title: &'static str,
    /// We store the Wikipedia article that we want to use for attribution. If a
    /// given poem doesn't have an appropriate Wikipedia article to link to,
    /// this will contain `None`.
    pub wiki: Option<&'static str>,
}

/// A list of epic poems
pub static EPICS: &[Epic] = &[
    // 8th to 6th century BC
    Epic {
        title: "Epic of Gilgamesh",
        wiki: Some("Epic of Gilgamesh"),
    },
    Epic {
        title: "Atrahasis",
        wiki: Some("Atra-Hasis"),
    },
    Epic {
        title: "Enuma Elish ",
        wiki: Some("Enuma Elish"),
    },
    Epic {
        title: "Legend of Keret",
        wiki: Some("Legend of Keret"),
    },
    Epic {
        title: "Cycle of Kumarbi",
        wiki: Some("Kumarbi"),
    },
    // 20th to 10th century BC
    Epic {
        title: "Iliad",
        wiki: Some("Iliad"),
    },
    Epic {
        title: "Odyssey",
        wiki: Some("Odyssey"),
    },
    Epic {
        title: "Works and Days",
        wiki: Some("Works and Days"),
    },
    Epic {
        title: "Theogony",
        wiki: Some("Theogony"),
    },
    Epic {
        title: "Shield of Heracles",
        wiki: Some("Shield of Heracles"),
    },
    Epic {
        title: "Catalogue of Women",
        wiki: Some("Catalogue of Women"),
    },
];
