//! Password strength checker
//!
//! The checker returns one of the following strengths:
//! `VeryWeak`, `Weak`, `Medium`, `Strong`, `VeryStrong`.
//! The password is evaluated on length and the characters it contains.

mod check_pass;
mod gen_pass;

pub use check_pass::{check_pass, PassStrength};
pub use gen_pass::{gen_pass, gen_pass_default, PassOptions};
