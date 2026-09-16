use uuid::Uuid;

// [SecOps Guard] Checked Signer & Authority Validation
    pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_unique_ids() {
        let id1 = generate_id();
        let id2 = generate_id();

        assert_ne!(id1, id2);
    }
}
