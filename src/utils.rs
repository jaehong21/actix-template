pub fn getenv(key: &str) -> String {
    std::env::var(key).expect(&format!("{} must be set", key))
}
