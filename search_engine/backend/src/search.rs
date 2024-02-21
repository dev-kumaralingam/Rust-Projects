use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use xorf::Xor8;

pub type Title = String;
pub type Url = String;
pub type Meta = Option<String>;
pub type PostId = (Title, Url, Meta);
pub type Filters = Vec<PostFilter>;

#[derive(Serialize, Deserialize)]
pub struct Storage {
    pub filters: Filters,
}

pub struct FilterProxy<K, B> {
    key: K,
    _phantom: std::marker::PhantomData<B>,
}

impl<K: Hash, B> FilterProxy<K, B> {
    pub fn new(key: K) -> Self {
        FilterProxy {
            key,
            _phantom: std::marker::PhantomData,
        }
    }
}

// Implement Serialize and Deserialize for FilterProxy
impl<K: Serialize, B> Serialize for FilterProxy<K, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.key.serialize(serializer)
    }
}

impl<'de, K: Deserialize<'de> + Hash, B> Deserialize<'de> for FilterProxy<K, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key = K::deserialize(deserializer)?;
        Ok(FilterProxy::new(key))
    }
}

pub type PostFilter = (PostId, FilterProxy<String, Xor8>);

pub fn score(title: &str, search_terms: &[String], filter: &FilterProxy<String, Xor8>) -> usize {
    let title_terms: Vec<String> = tokenize(title);
    let title_score: usize = search_terms
        .iter()
        .filter(|term| title_terms.contains(term))
        .count();
    TITLE_WEIGHT * title_score + score_xor(title, search_terms, filter)
}

fn score_xor(title: &str, search_terms: &[String], filter: &FilterProxy<String, Xor8>) -> usize {
    // Parse the filter key into an Xor8 instance
    let xor8_instance = Xor8::from_iterator(filter.key.bytes().map(|b| b as u64));

    // Your scoring logic here
    0 // Placeholder value
}

const TITLE_WEIGHT: usize = 3;

fn tokenize(s: &str) -> Vec<String> {
    s.to_lowercase()
        .split_whitespace()
        .filter(|&t| !t.trim().is_empty())
        .map(String::from)
        .collect()
}

pub fn search(filters: &Filters, query: String, num_results: usize) -> Vec<&PostId> {
    let search_terms: Vec<String> = tokenize(&query);
    let mut matches: Vec<(&PostId, usize)> = filters
        .iter()
        .map(|(post_id, filter)| (post_id, score(&post_id.0, &search_terms, filter)))
        .filter(|(_post_id, score)| *score > 0)
        .collect();

    matches.sort_by_key(|k| std::cmp::Reverse(k.1));

    matches.into_iter().take(num_results).map(|p| p.0).collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct HashProxy<K, H, B> {
    key: K,
    _hasher: H,
    _phantom: std::marker::PhantomData<B>,
}

impl<K: Hash, B> HashProxy<K, DefaultHasher, B> {
    pub fn new(key: K) -> Self {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        HashProxy {
            key,
            _hasher: hasher,
            _phantom: std::marker::PhantomData,
        }
    }
}

// Implement Serialize and Deserialize for HashProxy
impl<K: Serialize, B> Serialize for HashProxy<K, DefaultHasher, B> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.key.serialize(serializer)
    }
}

impl<'de, K: Deserialize<'de> + Hash, B> Deserialize<'de> for HashProxy<K, DefaultHasher, B> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key = K::deserialize(deserializer)?;
        Ok(HashProxy::new(key))
    }
}
