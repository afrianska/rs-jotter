// Listing 10-12 shows the definition of a public Summary trait that expresses this behavior.
#[allow(dead_code)]
pub trait Summary {
    fn summarize(&self) -> String;
}
