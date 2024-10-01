pub trait Morphology: Clone + Send + Sync + 'static {
    fn lookup(&self, fragment: &str) -> Option<&str>;
    fn lookup_with_state(&self, fragment: &str, state: usize) -> Option<&str>;
}
