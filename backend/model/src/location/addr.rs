use std::net::IpAddr;

pub enum LocationAddr {
    Ip(IpAddr),
    Domain(String),
}
