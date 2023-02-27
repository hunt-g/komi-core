use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// Wrapper for a string of space-separated values.
#[derive(Deserialize, Serialize)]
pub struct SpaceSeparatedValues(String);

impl SpaceSeparatedValues {
    // Split the string on whitespace and return a vector of strings.
    pub fn as_vec(&self) -> Vec<String> {
        self.0.split_whitespace().map(str::to_string).collect()
    }
}

impl std::fmt::Debug for SpaceSeparatedValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_vec())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats(std::collections::HashMap<String, String>);

/// Information about a single kanji character.
#[derive(Debug, Deserialize, Serialize)]
pub struct Kanji {
    /// Kanji Character
    pub character: String,
    /// String of space-separated onyomi readings for the kanji character.
    /// An empty string is treated as no readings.
    pub onyomi: String, // SpaceSeparatedValues,
    /// String of space-separated kunyomi readings for the kanji character.
    /// An empty string is treated as no readings.
    pub kunyomi: String, // SpaceSeparatedValues,
    /// String of space-separated tags for the kanji character.
    /// An empty string is treated as no tags.
    pub tags: String, // SpaceSeparatedValues,
    /// List of meanings for the kanji character.
    pub meanings: Vec<String>,
    /// Various stats for the kanji character.
    pub stats: Stats,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum KanjiMetaMode {
    Freq(u32),
    Pitch(serde_json::Value),
}

/// Metadata for a single kanji character.
#[derive(Debug, Deserialize, Serialize)]
pub struct KanjiMeta {
    /// Text for the kanji character.
    pub character: String,
    /// Mode for the kanji character.
    pub mode: String,
    /// Data for the kanji character.
    pub data: KanjiMetaMode,
}

/// Information about a single term.
#[derive(Debug, Deserialize, Serialize)]
pub struct Term {
    /// The text for the term.
    pub expression: String,
    /// Reading of the term, or an empty string if the reading is the same as the term.
    pub reading: String,
    /// String of space-separated tags for the definition.
    /// An empty string is treated as no tags.
    pub definition_tags: String, // SpaceSeparatedValues,
    /// String of space-separated rule identifiers for the definition which is used to validate delinflection.
    /// Valid rule identifiers are:
    /// - v1: ichidan verb;
    /// - v5: godan verb;
    /// - vs: suru verb;
    /// - vk: kuru verb;
    /// - adj-i: i-adjective.
    ///
    /// An empty string corresponds to words which aren't inflected, such as nouns.
    pub rules: String, // SpaceSeparatedValues,
    /// Score used to determine popularity.
    /// Negative values are more rare and positive values are more frequent.
    /// This score is also used to sort search results.
    pub score: i16,
    /// List of definitions for the term.
    pub glossary: Vec<String>,
    /// Sequence number for the term.
    /// Terms with the same sequence number can be shown together when the "resultOutputMode" option is set to "merge".
    pub sequence: u32,
    /// String of space-separated tags for the term.
    /// An empty string is treated as no tags.
    pub term_tags: String, // SpaceSeparatedValues,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum TermMetaMode {
    Freq(u32),
    Pitch(serde_json::Value),
}

/// Metadata for a single Term.
#[derive(Debug, Deserialize, Serialize)]
pub struct TermMeta {
    /// Text for the term.
    pub character: String,
    /// Type of data.
    /// "freq" corresponds to frequency information;
    /// "pitch" corresponds to pitch information.
    pub mode: String,
    /// Data for the term.
    pub data: TermMetaMode,
}

/// Information about a single tag.
#[derive(Debug, Deserialize, Serialize)]
pub struct Tag {
    /// Tag name.
    pub name: String,
    /// Category for the tag.
    pub category: String,
    /// Sorting order for the tag.
    pub order: i16,
    /// Notes for the tag.
    pub notes: String,
    /// Score used to determine popularity.
    /// Negative values are more rare and positive values are more frequent.
    /// This score is also used to sort search results.
    pub score: i16,
}

/// Index file containing information about the data contained in the dictionary.
#[derive(Debug, Deserialize, Serialize)]
pub struct Index {
    /// Title of the dictionary.
    pub title: String,
    /// Revision of the dictionary.
    /// This value is only used for displaying information.
    pub revision: String,
    /// Whether or not this dictionary contains sequencing information for related terms.
    pub sequenced: Option<bool>,
    /// Format of data found in the JSON data files.
    pub format: Option<u8>,
    /// Alias for format.
    pub version: Option<u8>,
    /// Creator of the dictionary.
    pub author: Option<String>,
    /// URL for the source of the dictionary.
    pub url: Option<String>,
    /// Description of the dictionary data.
    pub description: Option<String>,
    /// Attribution for the dictionary data.
    pub attribution: Option<String>,
    /// The frequency mode used for the dictionary.
    pub frequency_mode: Option<String>,
}

/// Container for all data in a single dictionary archive.
/// The data is stored within a zip file, in the following format:
/// - index.json: Contains information about the dictionary.
/// - term_bank_*.json: Each file contains a list of terms.
/// - kanji_bank_*.json: Each file contains a list of kanji.
/// - tag_meta_*.json: Each file contains a list of tag metadata.
/// - term_meta_*.json: Each file contains a list of term metadata.
/// - kanji_meta_*.json: Each file contains a list of kanji metadata.
///
/// (*) represent a number.
pub struct Dictionary {
    pub index: Index,
    pub kanji: Vec<Kanji>,
    pub kanji_meta: Vec<KanjiMeta>,
    pub terms: Vec<Term>,
    pub term_meta: Vec<TermMeta>,
    pub tag_meta: Vec<Tag>,
}

pub fn from_json<'a, M>(serialized: &String) -> M
where
    M: DeserializeOwned,
{
    serde_json::from_str(&serialized).expect("Failed to deserialize JSON")
}

impl Dictionary {
    pub fn new(path: &str) -> Dictionary {
        let mut index: Option<Index> = None;
        let mut kanji: Vec<Kanji> = Vec::new();
        let mut kanji_meta: Vec<KanjiMeta> = Vec::new();
        let mut terms: Vec<Term> = Vec::new();
        let mut term_meta: Vec<TermMeta> = Vec::new();
        let mut tag_meta: Vec<Tag> = Vec::new();

        print!("{}...", path);

        let mut archive = zip::ZipArchive::new(std::fs::File::open(path).unwrap()).unwrap();
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).unwrap();
            let filename = file.name().to_string();
            // Skip non-JSON files
            if !filename.ends_with(".json") {
                continue;
            }

            // Read the file into a string
            let mut contents = String::new();
            std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
            // Deserialize the JSON into the appropriate struct
            if filename == "index.json" {
                index = from_json(&contents);
            } else if filename.starts_with("kanji_bank_") {
                kanji.append(&mut from_json(&contents));
            } else if filename.starts_with("kanji_meta_bank_") {
                kanji_meta.append(&mut from_json(&contents));
            } else if filename.starts_with("term_bank_") {
                terms.append(&mut from_json(&contents));
            } else if filename.starts_with("term_meta_bank_") {
                term_meta.append(&mut from_json(&contents));
            } else if filename.starts_with("tag_bank_") {
                tag_meta.append(&mut from_json(&contents));
            } else {
                println!("Unknown file: {}", file.name());
            }
        }

        // Temporary outout formatting ()
        let mut counts: Vec<String> = Vec::new();

        let kanji_count = kanji.len();
        if kanji_count > 0 {
            counts.push(format!("{} kanji", kanji_count));
        }

        let kanji_meta_count = kanji_meta.len();
        if kanji_meta_count > 0 {
            counts.push(format!("{} kanji meta", kanji_meta_count));
        }

        let terms_count = terms.len();
        if terms_count > 0 {
            counts.push(format!("{} terms", terms_count));
        }

        let term_meta_count = term_meta.len();
        if term_meta_count > 0 {
            counts.push(format!("{} term meta", term_meta_count));
        }

        let tag_meta_count = tag_meta.len();
        if tag_meta_count > 0 {
            counts.push(format!("{} tags", tag_meta_count));
        }

        print!(" + {:} ", counts.join(", "));

        // Return the dictionary
        Dictionary {
            index: index.expect("No index.json file found in archive."),
            kanji,
            kanji_meta,
            terms,
            term_meta,
            tag_meta,
        }
    }
}
