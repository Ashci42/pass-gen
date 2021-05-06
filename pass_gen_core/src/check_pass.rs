use std::fmt;

/// Enum representing the strength of a password
///
/// `PassStrength` has 5 levels of strength
///
/// # Examples
/// ```
/// use pass_gen_core::PassStrength;
///
/// // strong password
/// let strong_password = PassStrength::Strong;
/// // very weak password
/// let very_weak_password = PassStrength::VeryWeak;
/// ```
#[derive(Debug, PartialEq)]
pub enum PassStrength {
    VeryWeak,
    Weak,
    Medium,
    Strong,
    VeryStrong,
}

impl fmt::Display for PassStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strength = match self {
            PassStrength::VeryWeak => "very weak",
            PassStrength::Weak => "weak",
            PassStrength::Medium => "medium",
            PassStrength::Strong => "strong",
            PassStrength::VeryStrong => "very strong",
        };

        write!(f, "{}", strength)
    }
}

impl From<u8> for PassStrength {
    fn from(points: u8) -> Self {
        match points {
            0 | 1 => PassStrength::VeryWeak,
            2 => PassStrength::Weak,
            3 => PassStrength::Medium,
            4 => PassStrength::Strong,
            _ => PassStrength::VeryStrong,
        }
    }
}

/// Checks the strength of `pass` and returns it.
///
/// `check_pass` looks for the following when checking the strength of `pass`
/// - length of `pass` (greater than 10 characters)
/// - contains lower case letters
/// - contains upper case letters
/// - contains digits
/// - contains special characters
///
/// # Examples
/// ```
/// use pass_gen_core::{check_pass, PassStrength};
///
/// let pass_strength = check_pass("Foo1");
///
/// assert_eq!(pass_strength, PassStrength::Medium);
/// ```
pub fn check_pass(pass: &str) -> PassStrength {
    let strength_points = calculate_strength_points(pass);

    PassStrength::from(strength_points)
}

fn calculate_strength_points(pass: &str) -> u8 {
    if pass.is_empty() {
        0
    } else {
        let mut points = 0;

        if pass.len() > 10 {
            points += 1;
        }

        if pass.chars().any(char::is_lowercase) {
            points += 1;
        }

        if pass.chars().any(char::is_uppercase) {
            points += 1;
        }

        if pass.chars().any(char::is_numeric) {
            points += 1;
        }

        if pass.contains('?') || pass.contains('!') || pass.contains('@') {
            points += 1
        }

        points
    }
}

#[cfg(test)]
mod check_pass_tests {
    use super::*;

    #[test]
    fn returns_medium_for_medium_passwords() {
        assert_eq!(check_pass("Foo1"), PassStrength::Medium);
    }

    #[test]
    fn returns_strong_for_strong_passwords() {
        assert_eq!(check_pass("Foo1!"), PassStrength::Strong);
    }

    #[test]
    fn returns_very_strong_for_very_strong_passwords() {
        assert_eq!(check_pass("Foo1!Bar2?96"), PassStrength::VeryStrong);
    }

    #[test]
    fn returns_very_weak_for_empty_passwords() {
        assert_eq!(check_pass(""), PassStrength::VeryWeak);
    }

    #[test]
    fn returns_very_weak_for_very_weak_passwords() {
        assert_eq!(check_pass("foo"), PassStrength::VeryWeak);
    }

    #[test]
    fn returns_weak_for_weak_passwords() {
        assert_eq!(check_pass("Foo"), PassStrength::Weak);
    }
}
