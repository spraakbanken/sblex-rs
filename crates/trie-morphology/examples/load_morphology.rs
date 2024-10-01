use std::{fs, io::BufWriter, time::SystemTime};

use trie_morphology::TrieMorphology;

fn main() -> eyre::Result<()> {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("assets/testing/saldo.lex"));
    let output = std::env::args().nth(2).unwrap_or_else(|| {
        let time = if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            n.as_secs()
        } else {
            0
        };
        format!("output.{:?}.json", time)
    });
    println!("loading from {} ...", input);
    let morph = TrieMorphology::from_path(&input)?;
    println!("{:#?}", morph.lookup("dväljes"));
    let file = fs::File::create(output)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &morph)?;
    Ok(())
}
