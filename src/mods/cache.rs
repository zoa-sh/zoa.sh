// PATH: src/mods/cache.rs
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

// Caching
pub struct Cacher;
#[rocket::async_trait]
impl Fairing for Cacher {
    fn info(&self) -> Info {
        Info {
            name: "Cacher",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.adjoin_header(Header::new("Cache-Control", "max-age=31536000"));
    }
}
