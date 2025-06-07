
use dotenv::dotenv;
use std::env;
use mysql::*;
pub fn db_connection() -> PooledConn {
    dotenv().ok();
    println!("DATABASE_URL from env: {:?}", env::var("DATABASE_URL"));

    let url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env");
    // let url = "mysql://root@202.74.74.133:3306/todo_list".to_string();


    let opts = Opts::from_url(&url).expect("Invalid database URL");
    let pool = Pool::new(opts).expect("Failed to create connection pool");

    pool.get_conn().expect("Failed to get connection")
}
