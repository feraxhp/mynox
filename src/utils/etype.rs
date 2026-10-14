
impl crate::Error {
    pub fn is_category(&self, prefix: &str) -> bool {
        let prefix = prefix.trim_end_matches(':');
        self.etype() == prefix || self.etype().starts_with(&format!("{prefix}::"))
    }
}

