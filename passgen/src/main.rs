use clap::{App, Arg, ArgGroup};
use pass_gen_core::{check_pass, gen_pass, gen_pass_default, PassOptions};

fn main() {
    let matches = App::new("passgen")
        .version("1.0.0")
        .author("Alex Olteanu")
        .about("Generates and checks passwords")
        .arg(
            Arg::with_name("length")
                .help("Should the password contain digits")
                .required(false)
                .short("e")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pass")
                .help("The password that needs to be checked")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::with_name("use_digits")
                .help("Should the password contain digits")
                .required(false)
                .short("d")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("use_lowercase_letters")
                .help("Should the password contain lowercase letters")
                .required(false)
                .short("l")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("use_special_chars")
                .help("Should the password contain special characters")
                .required(false)
                .short("s")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("use_uppercase_letters")
                .help("Should the password contain uppercase letters")
                .required(false)
                .short("u")
                .takes_value(false),
        )
        .group(ArgGroup::with_name("check").args(&["pass"]).required(false))
        .group(
            ArgGroup::with_name("gen")
                .args(&[
                    "length",
                    "use_digits",
                    "use_lowercase_letters",
                    "use_uppercase_letters",
                    "use_special_chars",
                ])
                .required(false)
                .conflicts_with("check")
                .multiple(true),
        )
        .get_matches();

    if matches.is_present("check") {
        let pass = matches.value_of("pass").unwrap();
        let pass_strength = check_pass(pass);

        println!(
            "{pass} is {strength}",
            pass = pass,
            strength = pass_strength
        );
    } else {
        let pass;

        if matches.is_present("gen") {
            let length = matches.value_of("length");
            let use_digits = matches.is_present("use_digits");
            let use_lowercase = matches.is_present("use_lowercase_letters");
            let use_special_chars = matches.is_present("use_special_chars");
            let use_uppercase = matches.is_present("use_uppercase_letters");
            let pass_options;

            if let Some(length) = length {
                match length.parse::<u8>() {
                    Ok(length) => {
                        pass_options = PassOptions::new(
                            length,
                            use_digits,
                            use_lowercase,
                            use_special_chars,
                            use_uppercase,
                        );
                    }
                    Err(_) => {
                        println!("Length should be a positive number, instead got {}", length);
                        return;
                    }
                }
            } else {
                pass_options = PassOptions::with_default_length(
                    use_digits,
                    use_lowercase,
                    use_special_chars,
                    use_uppercase,
                )
            }

            pass = gen_pass(pass_options);
        } else {
            pass = Some(gen_pass_default());
        }

        if let Some(pass) = pass {
            println!("{}", pass);
        } else {
            println!("Failed to generate password");
        }
    }
}
