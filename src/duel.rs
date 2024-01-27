pub mod duel {
    use chrono::{Local, DateTime};
    use std::str::FromStr;
    use anyhow::{anyhow, Context, Result};
    #[derive(Debug, Clone)]
    pub struct TwitchUserId(String);

    impl FromStr for TwitchUserId {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            dbg!(s);
            if s.chars().all(|ch| ch.is_alphanumeric()) {
                Ok(TwitchUserId(String::from(s)))
            } else {
                Err(anyhow!("valid handles only contain characters 0-9 and a-f"))
            }
        }
    }

    impl std::fmt::Display for TwitchUserId {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    pub struct Duel {
        challenge_datetime: DateTime<Local>,
        
        pub challenger: TwitchUserId, 
        
        pub challenged: TwitchUserId,
        
        points: u32,

        pub winner: TwitchUserId, 

        accepted: bool
    }

    impl Duel {
        pub fn new(challenger: &str, challenged: &str, points: u32) -> Duel {

            let dt = Local::now();

            Duel {
                challenge_datetime: dt,
                challenger: TwitchUserId::from_str(challenger).unwrap(),
                challenged: TwitchUserId::from_str(challenger).unwrap(),
                points: points,
                winner: TwitchUserId::from_str("").unwrap(),
                accepted: false
            }
        }
    }

}
