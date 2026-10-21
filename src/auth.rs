pub struct TokenValidator {
    pub max_skew_secs: u64,
    pub max_age_secs: u64,
}

impl TokenValidator {
    pub fn new(max_skew_secs: u64, max_age_secs: u64) -> Self {
        Self {
            max_skew_secs,
            max_age_secs,
        }
    }

    pub fn check_timestamp(&self, issued_at: u64, now: u64) -> bool {
        if issued_at > now.saturating_add(self.max_skew_secs) {
            return false;
        }
        now.saturating_sub(issued_at) <= self.max_age_secs
    }
}
