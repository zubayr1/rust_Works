fn next_lang<'a>(langs: &'a [String], current: &str) -> &'a str {
    let mut found = false;

    for lang in langs {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    return langs.last().unwrap();
}

fn main() {
    let langs = vec![
        "rust".to_string(),
        "C#".to_string(),
        "ts".to_string(),
        "go".to_string(),
        "python".to_string(),
        "js".to_string(),
    ];

    let result = next_lang(&langs, &"ts");

    println!("{}", result);
}
