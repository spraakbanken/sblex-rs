use sblex::fm::Morphology;

#[test]
fn load_morphology_from_file() {
    let morph = Morphology::from_path("../assets/testing/dalin.lex").unwrap();

    let result = morph.lookup("ö", 0);

    let expected = Some("{\"a\":[],\"c\":\"gsmkr\"}");

    assert_eq!(result, expected);

    let result = morph.lookup("ögna", 0);

    let expected = Some("{\"a\":[{\"gf\":\"ögna\",\"id\":\"dalinm--ögna..vb.1\",\"is\":[],\"msd\":\"-\",\"p\":\"vb\",\"pos\":\"vb\"}],\"c\":\"\"}");

    assert_eq!(result, expected);
}
