use sblex::fm::Morphology;

#[test]
fn load_morphology_from_file() {
    let morph = Morphology::from_path("../assets/testing/dalin.lex").unwrap();

    let result = morph.lookup("ö", 0);

    let expected = Some("öga");

    assert_eq!(result, expected);
}
