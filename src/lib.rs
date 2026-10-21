pub mod auth;
pub mod limits;

pub const PROTOCOL_VERSION: u32 = 1;

pub fn verify_version(v: u32) -> bool {
    v == PROTOCOL_VERSION
}
