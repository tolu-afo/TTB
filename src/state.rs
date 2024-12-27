use crate::models;
use models::Duel;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct State {
    pub duel_cache: HashMap<String, VecDeque<Duel>>,
}

impl State {
    pub fn new() -> State {
        let duel_cache: HashMap<String, VecDeque<Duel>> = HashMap::new();

        return State {
            duel_cache: duel_cache,
        };
    }

    pub fn save_duel(&mut self, duel: &Duel) -> () {
        // saves duel to cache
        // TODO: Fix this or scrap and start it over.

        let k = format!(
            "{}{}",
            duel.challenger.to_lowercase(),
            duel.challenged.to_lowercase()
        );

        if self.duel_cache.contains_key(&k) {
            self.duel_cache.get_mut(&k).unwrap().push_back(duel.clone());
        } else {
            let mut q = VecDeque::new();
            q.push_back(duel.clone());
            self.duel_cache.insert(
                format!(
                    "{}{}",
                    duel.challenger.to_lowercase(),
                    duel.challenged.to_lowercase()
                ),
                q,
            );
        }
    }

    pub fn get_duel(&mut self, k: &String) -> Option<Duel> {
        match self.duel_cache.get_mut(k) {
            Some(q) => q.pop_front(),
            None => None,
        }
    }

    pub fn clear_duel(&mut self, duel: &Duel) -> bool {
        let k = format!(
            "{}{}",
            duel.challenger.to_lowercase(),
            duel.challenged.to_lowercase()
        );
        match self.duel_cache.get_mut(&k) {
            Some(q) => match q.pop_front() {
                Some(duel) => return true,
                None => return false,
            },
            None => return false,
        }
    }
}
