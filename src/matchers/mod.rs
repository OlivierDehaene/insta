pub trait Matcher {
    fn match_contents(&self, a: &str, b: &str) -> bool;
}

pub struct DefaultMatcher;

impl Matcher for DefaultMatcher {
    fn match_contents(&self, a: &str, b: &str) -> bool {
        a.trim_end() == b.trim_end()
    }
}

#[cfg(feature = "yaml")]
mod yaml_matcher;

#[cfg(feature = "yaml")]
pub use yaml_matcher::YamlMatcher;
