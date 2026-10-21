pub mod auth;
pub mod limits;

pub fn verify_version(v: u32) -> bool {
    v == 1
}
