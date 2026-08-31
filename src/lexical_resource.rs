#[derive(Debug, yaserde::YaDeserialize)]
pub struct LexicalResource {
    #[yaserde(rename = "GlobalInformation")]
    global_information: GlobalInformation,
    #[yaserde(rename = "Lexicon")]
    lexicon: Lexicon,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct GlobalInformation {
    feat: Feat,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Lexicon {
    feat: Feat,
    #[yaserde(rename = "LexicalEntry")]
    entries: Vec<LexicalEntry>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct LexicalEntry {
    #[yaserde(rename = "Lemma")]
    lemma: Lemma,
    #[yaserde(rename = "WordForm")]
    word_form: Option<WordForm>,
    #[yaserde(rename = "Sense")]
    senses: Vec<Sense>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Lemma {
    #[yaserde(rename = "FormRepresentation")]
    form_representation: FormRepresentation,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct FormRepresentation {
    #[yaserde(rename = "feat")]
    feats: Vec<Feat>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct WordForm {
    #[yaserde(rename = "feat")]
    feats: Vec<Feat>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Sense {
    #[yaserde(attribute = true)]
    id: String,
    #[yaserde(rename = "Definition")]
    definition: Option<Definition>,
}

#[derive(Debug, yaserde::YaDeserialize)]
pub struct Definition {
    #[yaserde(rename = "feat")]
    feats: Vec<Feat>,
}
#[derive(Debug, yaserde::YaDeserialize)]
pub struct Feat {
    #[yaserde(attribute = true)]
    att: String,
    #[yaserde(attribute = true)]
    val: String,
}
