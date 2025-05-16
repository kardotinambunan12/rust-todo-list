use serde::{Deserialize, Serialize};

use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub sub:String,
    pub exp:usize,
}

impl Claims {
    pub fn new(email:&str)->Self{
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(60))
            .expect("invalid timestamp")
            .timestamp();

        Claims{
            sub:email.to_owned(),
            exp:expiration as usize,
        }
    }

}
