#[derive(Debug, yaserde::YaDeserialize)]
pub struct LexicalResource {
    #[yaserde(rename = "GlobalInformation")]
    global_information: GlobalInformation,
    #[yaserde(rename = "Lexicon")]
    pub lexicon: Lexicon,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct GlobalInformation {
    feat: Feat,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Lexicon {
    feat: Feat,
    #[yaserde(rename = "LexicalEntry")]
    pub entries: Vec<LexicalEntry>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct LexicalEntry {
    #[yaserde(rename = "Lemma")]
    pub lemma: Lemma,
    #[yaserde(rename = "WordForm")]
    pub word_form: Option<WordForm>,
    #[yaserde(rename = "Sense")]
    pub senses: Vec<Sense>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Lemma {
    #[yaserde(rename = "FormRepresentation")]
    pub form_representation: FormRepresentation,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct FormRepresentation {
    #[yaserde(rename = "feat")]
    pub feats: Vec<Feat>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct WordForm {
    #[yaserde(rename = "feat")]
    pub feats: Vec<Feat>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Sense {
    #[yaserde(attribute = true)]
    pub id: String,
    #[yaserde(rename = "Definition")]
    pub definition: Option<Definition>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Definition {
    #[yaserde(rename = "feat")]
    pub feats: Vec<Feat>,
}
#[derive(Debug, yaserde::YaDeserialize)]
pub struct Feat {
    #[yaserde(attribute = true)]
    pub att: String,
    #[yaserde(attribute = true)]
    pub val: String,
}
