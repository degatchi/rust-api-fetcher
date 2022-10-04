use serde_json::Value;
use std::collections::HashMap;

pub struct RickAndMorty {
    base_url: String,
    sub_urls: HashMap<String, String>,
    characters: HashMap<u64, Value>,
    episodes: HashMap<u64, Value>,
    locations: HashMap<u64, Value>,
}

impl RickAndMorty {
    pub fn new() -> Self {
        Self {
            base_url: "https://rickandmortyapi.com/api".to_owned(),
            sub_urls: HashMap::new(),
            characters: HashMap::new(),
            episodes: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    pub async fn get_sub_urls(&mut self) {
        self.sub_urls = reqwest::get("https://rickandmortyapi.com/api")
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();
        println!("sub urls: {:?}", self.sub_urls);
    }

    pub async fn get_data(&mut self) {
        self.get_sub_urls().await;
        self.get_characters().await;
        self.get_episodes().await;
        self.get_locations().await;
    }

    async fn get_characters(&mut self) {
        let characters_api = self.sub_urls.get("characters").unwrap();

        // Replace 43 w/ dynamic
        for i in 1..43 {
            // print!("//////////// Page: {}/42 ////////////\n", &i);
            let url = format!("{}?page={}", characters_api, i);
            let found_characters = reqwest::get(url)
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            if let Some(results) = found_characters["results"].as_array() {
                // println!("Len: {:#?}", &results.len());
                for character in results.iter() {
                    let id = character["id"].as_u64().unwrap();
                    self.characters.insert(id, character.clone());
                }
                // println!("{:#?}", self.characters.len());
            }

            if let Some(info) = found_characters["info"].as_object() {
                if info["next"] == Value::Null {
                    // println!("No more pages");
                }
            }
        }
    }

    async fn get_episodes(&mut self) {
        let episodes_api = self.sub_urls.get("episodes").unwrap();

        for i in 1..4 {
            // print!("//////////// Page: {}/42 ////////////\n", &i);
            let url = format!("{}?page={}", episodes_api, i);
            let found_episode = reqwest::get(url)
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            if let Some(results) = found_episode["results"].as_array() {
                // println!("Len: {:#?}", &results.len());
                for episode in results.iter() {
                    let id = episode["id"].as_u64().unwrap();
                    self.episodes.insert(id, episode.clone());
                }
                // println!("{:#?}", self.characters.len());
            }

            if let Some(info) = found_episode["info"].as_object() {
                if info["next"] == Value::Null {
                    // println!("No more pages");
                }
            }
        }
        println!("total episodes: {}", self.episodes.len());
    }

    async fn get_locations(&mut self) {
        let locations_api = self.sub_urls.get("locations").unwrap();

        for i in 1..8 {
            // print!("//////////// Page: {}/42 ////////////\n", &i);
            let url = format!("{}?page={}", locations_api, i);
            let found_locations = reqwest::get(url)
                .await
                .unwrap()
                .json::<Value>()
                .await
                .unwrap();

            if let Some(results) = found_locations["results"].as_array() {
                // println!("Len: {:#?}", &results.len());
                for location in results.iter() {
                    let id = location["id"].as_u64().unwrap();
                    self.locations.insert(id, location.clone());
                }
                // println!("{:#?}", self.characters.len());
            }

            if let Some(info) = found_locations["info"].as_object() {
                if info["next"] == Value::Null {
                    // println!("No more pages");
                }
            }
        }
        println!("total locations: {}", self.locations.len());
    }
}
