use fjall_morphology::FjallMorphology;
use sblex_services::{morphology, ports::Morphology, MorphologyBuilder};
use temp_dir::TempDir;

#[test]
fn build_and_load_morphology() -> eyre::Result<()> {
    let tmp_dir = TempDir::with_prefix("test.db")?;
    let mut morph = FjallMorphology::new(tmp_dir.path())?;

    morphology::build_from_path(&mut morph, "assets/testing/saldo.lex")?;
    morph.finish()?;

    let result = morph.lookup("dv")?;
    assert!(result.is_none());

    let result = morph.lookup("dväljs")?.unwrap();
    let result_json: serde_json::Value = serde_json::from_slice(&result)?;
    insta::assert_json_snapshot!("lookup__dväljs", result_json);

    let result = morph.lookup_with_cont("dv")?;
    let result_json: serde_json::Value = serde_json::from_slice(&result)?;
    insta::assert_json_snapshot!("lookup_with_cont__dv", result_json);

    let result = morph.lookup_with_cont("dväljs")?;
    let result_json: serde_json::Value = serde_json::from_slice(&result)?;
    insta::assert_json_snapshot!("lookup_with_cont__dväljs", result_json);
    Ok(())
}
