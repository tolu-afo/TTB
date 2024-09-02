use crate::chatter::TwitchUserId;
use crate::models;
use models::Duel;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    pub duel_cache: HashMap<String, Duel>,
}

impl State {
    pub fn new() -> State {
        let duel_cache: HashMap<String, Duel> = HashMap::new();
        let chatter_cache: HashMap<String, TwitchUserId> = HashMap::new();

        return State {
            duel_cache: duel_cache,
            // chatter_cache: chatter_cache,
        };
    }

    pub fn save_duel(&mut self, duel: &Duel) -> () {
        // saves duel to cache
        // TODO: Fix this or scrap and start it over.
        self.duel_cache.insert(
            format!(
                "{}{}",
                duel.challenger.to_lowercase(),
                duel.challenged.to_lowercase()
            ),
            duel.clone(),
        );
    }
}
