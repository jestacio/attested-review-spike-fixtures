pub struct TokenValidator {
    pub max_skew_secs: u64,
}

impl TokenValidator {
    pub fn new(max_skew_secs: u64) -> Self {
        Self { max_skew_secs }
    }

    pub fn check_timestamp(&self, issued_at: u64, now: u64) -> bool {
        issued_at <= now + self.max_skew_secs
    }
}
