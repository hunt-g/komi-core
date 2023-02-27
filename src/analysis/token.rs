// /// Structure representing a token.
// pub struct Token<'a> {
//     /// Text content of the token.
//     pub text: &'a str,

//     /// Starting position of the token in bytes.
//     pub byte_start: usize,

//     /// Ending position of the token in bytes.
//     pub byte_end: usize,

//     /// Reference of dictionary.
//     // pub dictionary: &Dictionary,

//     /// Metadata for the token.
//     details: Option<Vec<String>>,
// }

// impl Token<'_> {
//     pub fn new(
//         text: &str,
//         byte_start: usize,
//         byte_end: usize,
//         // dictionary: &Dictionary,
//     ) -> Self {
//         Self {
//             text,
//             byte_start,
//             byte_end,
//             // dictionary,
//             details: None,
//         }
//     }

//     pub fn get_details(&self) -> Option<Vec<String>> {
//         self.details
//     }

//     pub fn set_details(&mut self, details: Vec<String>) {
//         self.details = Some(details);
//     }
// }
