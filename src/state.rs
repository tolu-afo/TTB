pub mod state {

    use crate::duel;

    #[derive(Debug)]
    pub struct State {
        duel_cache: Vec::<duel::duel::Duel> 
    }

    impl State {
        pub fn new() -> State {
            let cache:Vec::<duel::duel::Duel> = vec![];

            return State {
                duel_cache: cache,
            }
        }

        pub fn save_duel(&mut self, duel: duel::duel::Duel) -> (){
            // saves duel to cache
            self.duel_cache.push(duel);
        }
    }
}