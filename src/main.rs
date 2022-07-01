use genanki_rs::{basic_model, Deck, Note};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let words = parse("source/000099579.csv")?;
    let mut deck = Deck::new(1, "セキュリティ英単語集", "セキュリティ関係の単語330語");

    for word in words {
        deck.add_note(Note::new(
            basic_model(),
            word.iter().map(|s| s.as_str()).collect(),
        )?);
    }

    deck.write_to_file("security-english-words.apkg")?;
    Ok(())
}

fn parse(path: impl AsRef<Path>) -> anyhow::Result<Vec<Vec<String>>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut words = Vec::new();
    for result in reader.records() {
        let record = result?;
        let note = match (record.get(0), record.get(1)) {
            (Some(english), Some(japanese)) => vec![english.to_string(), japanese.to_string()],
            _ => continue,
        };
        words.push(note);
    }
    Ok(words)
}
