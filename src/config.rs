use mqtt::{Filter, FilterBuf, Topic, TopicBuf};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ConfigOwned {
    pub url: String,
    pub filter: FilterBuf,
    pub mappings: HashMap<String, Vec<MappingOwned>>,
}

pub struct Config<'a> {
    pub url: &'a str,
    pub filter: &'a Filter,
    pub mappings: HashMap<&'a str, Vec<Mapping<'a>>>,
}

#[derive(Deserialize)]
pub struct MappingOwned {
    pub topic: TopicBuf,
    pub payload: String,
}

pub struct Mapping<'a> {
    pub topic: &'a Topic,
    pub payload: &'a str,
}

impl ConfigOwned {
    // This is cursed.
    pub fn leak(self) -> Config<'static> {
        let url: &'static _ = self.url.leak();
        let filter: &'static _ = Filter::from_static(self.filter.to_inner().leak());
        let mappings: HashMap<_, _> = self
            .mappings
            .into_iter()
            .map(|(key, value)| {
                let key: &'static str = key.leak();
                let mappings: Vec<_> = value
                    .into_iter()
                    .map(|mapping| {
                        let topic: &'static Topic =
                            Topic::from_static(mapping.topic.to_inner().leak());
                        let payload: &'static str = mapping.payload.leak();

                        Mapping { topic, payload }
                    })
                    .collect();

                (key, mappings)
            })
            .collect();

        Config {
            url,
            filter,
            mappings,
        }
    }
}
