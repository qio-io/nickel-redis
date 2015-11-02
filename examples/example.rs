extern crate r2d2;
extern crate redis;
extern crate openssl;
#[macro_use] extern crate nickel;
extern crate nickel_redis;

use std::env;
use r2d2::NopErrorHandler;
use nickel::{Nickel, HttpRouter};
use nickel_redis::{RedisMiddleware, RedisRequestExtensions};

fn main() {
    let mut app = Nickel::new();

    let redis_url = env::var("REDIS_URL").unwrap();
    let redispool = RedisMiddleware::new(&*redis_url,
                                         Box::new(NopErrorHandler)).unwrap();

    app.utilize(redispool);
    app.get("/my_counter", middleware! { |request|
        let _connection = request.redis_conn();

        // use connection
    });

    app.get("**", middleware! { println!("!!!") });
}
