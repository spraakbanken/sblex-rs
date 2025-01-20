use std::{fs, io::BufWriter, time::SystemTime};

use sblex_services::morphology;
use trie_morphology::{trie::TrieBuilder, TrieMorphology};

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
    let mut morph_builder = TrieBuilder::default();
    morphology::build_from_path(&mut morph_builder, &input)?;
    let morph = TrieMorphology::new(morph_builder.build());
    println!("{:#?}", morph.lookup("dväljes"));
    let file = fs::File::create(output)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &morph)?;
    Ok(())
}
