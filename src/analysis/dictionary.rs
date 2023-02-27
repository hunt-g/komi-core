// Dictionary for morphological analysis.

pub struct Dictionary {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    pub url: String,
    pub entries: Vec<String>,
}

impl Dictionary {
    pub fn new(
        name: &String,
        version: &String,
        author: &String,
        description: &String,
        license: &String,
        url: &String,
    ) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            author: author.to_string(),
            description: description.to_string(),
            license: license.to_string(),
            url: url.to_string(),
            entries: Vec::new(),
        }
    }
}
