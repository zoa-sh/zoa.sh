// PATH: src/mods/agent.rs
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Data;
use rocket::Request;

pub struct UserAgentFairing;

#[rocket::async_trait]
impl Fairing for UserAgentFairing {
    fn info(&self) -> Info {
        Info {
            name: "User-Agent Checker",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        if let Some(user_agent) = request.headers().get_one("User-Agent") {
            if user_agent.contains("curl") {
                let path = request.uri().path(); // we need to modify this to parse /text for paths
                if path == "/" {
                    request.set_uri(request.uri().map_path(|_| "/text").unwrap());
                } else if path == "/net" {
                    request.set_uri(request.uri().map_path(|_| "/net").unwrap());
                }
            }
        }
    }
}
