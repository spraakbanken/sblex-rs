use sblex::fm::Morphology;

fn main() {
    let mut args = std::env::args().skip(1);
    let path: Option<String>= args.next();
    let path = path.as_ref().map(|s| s.as_str()).unwrap_or("assets/dalin.lex");
    let morph = Morphology::from_path(path).unwrap();
    println!("{:?}", morph.lookup("ösja"));
}

