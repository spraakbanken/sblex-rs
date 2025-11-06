use sblex_services::{morphology, ports::Morphology};
use trie_morphology::{trie::TrieBuilder, TrieMorphology};

#[test]
fn load_morphology_from_file() -> eyre::Result<()> {
    let mut morph_builder = TrieBuilder::default();
    morphology::build_from_path(&mut morph_builder, "../../assets/testing/dalin.lex")?;
    let morph = TrieMorphology::new(morph_builder.build());

    let result = Morphology::lookup(&morph, "ö").unwrap();
    dbg!(&result);
    assert!(result.is_none());
    let result = morph.lookup_with_cont("ö").unwrap();
    let result: serde_json::Value = serde_json::from_slice(&result)?;
    let mut c_chars: Vec<char> = result["c"].as_str().unwrap().chars().collect();
    c_chars.sort();
    let c_result = c_chars.into_iter().collect::<String>();

    let c_expected = "gkmrs";

    assert_eq!(c_result, c_expected);

    let result = morph.lookup_with_cont("ögna").unwrap();

    let result = Some(std::str::from_utf8(&result).unwrap());
    let expected = Some("{\"a\":[{\"gf\":\"ögna\",\"id\":\"dalinm--ögna..vb.1\",\"is\":[],\"msd\":\"-\",\"p\":\"vb\",\"pos\":\"vb\"}],\"c\":\"\"}");

    assert_eq!(result, expected);
    let result = Morphology::lookup(&morph, "ögna").unwrap();

    assert!(result.is_some());
    let result = result.unwrap();
    let result = Some(std::str::from_utf8(&result).unwrap());
    let expected = Some("[{\"gf\":\"ögna\",\"id\":\"dalinm--ögna..vb.1\",\"is\":[],\"msd\":\"-\",\"p\":\"vb\",\"pos\":\"vb\"}]");

    assert_eq!(result, expected);
    Ok(())
}
