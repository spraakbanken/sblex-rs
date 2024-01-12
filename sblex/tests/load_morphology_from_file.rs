use sblex::fm::Morphology;

#[test]
fn load_morphology_from_file() -> eyre::Result<()> {
    let morph = Morphology::from_path("../assets/testing/dalin.lex").unwrap();

    let result = morph.lookup("ö").unwrap();
    let result: serde_json::Value = serde_json::from_str(result)?;
    let mut c_chars: Vec<char> = result["c"].as_str().unwrap().chars().collect();
    c_chars.sort();
    let c_result = c_chars.into_iter().collect::<String>();

    let c_expected = "gkmrs";

    assert_eq!(c_result, c_expected);

    let result = morph.lookup("ögna");

    let expected = Some("{\"a\":[{\"gf\":\"ögna\",\"id\":\"dalinm--ögna..vb.1\",\"is\":[],\"msd\":\"-\",\"p\":\"vb\",\"pos\":\"vb\"}],\"c\":\"\"}");

    assert_eq!(result, expected);
    Ok(())
}
