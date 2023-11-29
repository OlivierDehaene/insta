use std::fmt;
use std::marker::PhantomData;
use serde::de::DeserializeOwned;
use crate::matchers::Matcher;

pub struct YamlMatcher<T> {
    _phantom_data: PhantomData<T>,
}

impl<T> YamlMatcher<T> {
    pub fn new() -> Self {
        Self {
            _phantom_data: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + fmt::Debug + PartialEq> Matcher for YamlMatcher<T> {
    fn match_contents(&self, a: &str, b: &str) -> bool {
        let value_a: T = serde_yaml::from_str(a).unwrap();
        let value_b: T = serde_yaml::from_str(b).unwrap();
        value_a == value_b
    }
}