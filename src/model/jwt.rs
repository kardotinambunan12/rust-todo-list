use serde::{Deserialize, Serialize};

use chrono::{Utc, Duration};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims{
    pub email:String,
    pub exp:usize,

}

impl Claims {
    pub fn new(email:&str)->Self{
        let expiration = Utc::now()
            .checked_add_signed(Duration::minutes(60))
            .expect("invalid timestamp")
            .timestamp();

        Claims{
            email:email.to_owned(),
            exp:expiration as usize,
        }
    }

}
