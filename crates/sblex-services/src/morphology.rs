pub trait Morphology {
    fn lookup(&self, fragment: &str) -> Option<&str>;
}
