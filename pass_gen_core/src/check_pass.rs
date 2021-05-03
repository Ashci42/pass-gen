pub enum PassStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

pub fn check_pass(_pass: &str) -> PassStrength {
    PassStrength::Medium
}
