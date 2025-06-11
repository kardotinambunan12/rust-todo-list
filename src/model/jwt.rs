use serde::{Deserialize, Serialize};

use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub email:String,
    pub exp:usize,

}

impl Claims {
    pub fn new(email:&str, duration: Duration)->Self{
        let expiration = Utc::now()
            .checked_add_signed(duration)
            .expect("invalid timestamp")
            .timestamp();

        Claims{
            email:email.to_owned(),
            exp:expiration as usize,
        }
    }

}
