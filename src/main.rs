use komi_core::dictionary::yomichan::Dictionary;
use std::time::Instant;

fn test_imports() {
    let mut dicts: Vec<Dictionary> = Vec::new();

    let archives: [&str; 17] = [
        "innocent_corpus.zip",
        "jmdict_dutch.zip",
        "jmdict_english.zip",
        "jmdict_french.zip",
        "jmdict_german.zip",
        "jmdict_hungarian.zip",
        "jmdict_russian.zip",
        "jmdict_slovenian.zip",
        "jmdict_spanish.zip",
        "jmdict_swedish.zip",
        "jmnedict.zip",
        "kanjidic_english.zip",
        "kanjidic_french.zip",
        "kanjidic_portuguese.zip",
        "kanjidic_spanish.zip",
        "kanjium_pitch_accents.zip",
        "kireicake.zip",
    ];

    for archive in archives.iter() {
        let start = Instant::now();
        let dict = Dictionary::new(archive);
        dicts.push(dict);
        let duration = start.elapsed();
        println!("in {:?}s", duration.as_secs_f32());
    }

    print!("\nFinished! Imported {:?} dictionaries ", dicts.len());
}

fn main() {
    let start = Instant::now();
    test_imports();
    let duration = start.elapsed();
    println!("in {:?}s", duration.as_secs_f32());
    std::process::exit(0);
}
