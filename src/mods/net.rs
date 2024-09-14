// PATH: src/mods/net.rs
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use std::net::IpAddr;

// Network tools
pub struct ClientIp(IpAddr);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ClientIp {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        match request.client_ip() {
            Some(ip) => Outcome::Success(ClientIp(ip)),
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}
#[get("/net")]
pub fn ip(client_ip: ClientIp) -> String {
    format!("IP Address: {}", client_ip.0) // "net/ip"
}
