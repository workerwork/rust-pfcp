use dotenv::dotenv;
use redis::Commands;
use std::env;

pub fn get() -> redis::RedisResult<isize> {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").unwrap();
    // connect to redis
    //let client = redis::Client::open("redis://127.0.0.1/")?;
    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _: () = con.set("my_key", 429)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}

//todo redis pool
