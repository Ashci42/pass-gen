use random_string::{generate, Charset};

/// Options used to generate a password
///
/// `PassOptions` provides some basic options for generating a password
///
/// # Examples
/// ```
/// use pass_gen_core::PassOptions;
///
/// let pass_options = PassOptions {
///     length: 8,
///     use_digits: true,
///     use_lowercase: true,
///     use_special_chars: false,
///     use_uppercase: false,   
/// };
/// ```
pub struct PassOptions {
    length: u8,
    use_digits: bool,
    use_lowercase: bool,
    use_special_chars: bool,
    use_uppercase: bool,
}

impl PassOptions {
    pub fn new(
        length: u8,
        use_digits: bool,
        use_lowercase: bool,
        use_special_chars: bool,
        use_uppercase: bool,
    ) -> Self {
        Self {
            length,
            use_digits,
            use_lowercase,
            use_special_chars,
            use_uppercase,
        }
    }

    pub fn with_default_length(
        use_digits: bool,
        use_lowercase: bool,
        use_special_chars: bool,
        use_uppercase: bool,
    ) -> Self {
        Self {
            use_digits,
            use_lowercase,
            use_special_chars,
            use_uppercase,
            ..Default::default()
        }
    }

    fn get_character_set(&self) -> String {
        let mut character_set = String::from("");

        if self.use_digits {
            character_set += "123456789";
        }

        if self.use_lowercase {
            character_set += "abcdefghijklmnopqrstuvwxyz";
        }

        if self.use_special_chars {
            character_set += "!?#";
        }

        if self.use_uppercase {
            character_set += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        }

        character_set
    }
}

impl Default for PassOptions {
    fn default() -> Self {
        PassOptions {
            length: 16,
            use_digits: true,
            use_lowercase: true,
            use_special_chars: true,
            use_uppercase: true,
        }
    }
}

/// Generates a password.
///
/// Depending on the `options` the password may contain the following:
/// - lowercase letters
/// - uppercase letters
/// - digits
/// - special characters
/// The options can also specify a preferred password length.
/// Returns `None` if the criteria in the options are all false
///
/// # Examples
/// ```
/// use pass_gen_core::{gen_pass, PassOptions};
///
/// let pass_options = PassOptions {
///     length: 8,
///     use_digits: true,
///     use_lowercase: true,
///     use_special_chars: false,
///     use_uppercase: false,   
/// };
/// let pass = gen_pass(pass_options);
/// ```
pub fn gen_pass(pass_options: PassOptions) -> Option<String> {
    let character_set = pass_options.get_character_set();
    let character_set = Charset::new(character_set);

    character_set
        .map(|character_set| generate(pass_options.length as usize, &character_set).to_string())
}

/// Generates a password.
///
/// `gen_pass_default` is different from `gen_pass`. It does not take
/// a `PassOptions` argument because it uses its default.
///
/// # Examples
/// ```
/// use pass_gen_core::gen_pass_default;
///
/// let pass = gen_pass_default();
/// ```
pub fn gen_pass_default() -> String {
    gen_pass(PassOptions::default()).unwrap()
}
