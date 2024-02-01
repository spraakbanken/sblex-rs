use sblex::fm::Morphology;

fn main() -> eyre::Result<()> {
    let input = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("assets/testing/saldo.lex"));

    println!("loading from {} ...", input);
    let _morph = Morphology::from_path(input)?;
    Ok(())
}
