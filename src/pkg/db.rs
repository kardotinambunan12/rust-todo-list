
use dotenv::dotenv;
use std::env;
use mysql::*;
pub fn db_connection() -> PooledConn {
    dotenv().ok();
    println!("DATABASE_URL from env: {:?}", env::var("DATABASE_URL"));

    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");


    let opts = Opts::from_url(&url).expect("Invalid database URL");
    let pool = Pool::new(opts).expect("Failed to create connection pool");

    pool.get_conn().expect("Failed to get connection")
}
