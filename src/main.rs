use std::{
    io::{self, BufRead},
};
use strsim::{
    hamming,
    levenshtein,
    normalized_levenshtein,
    osa_distance,
    damerau_levenshtein,
    normalized_damerau_levenshtein,
    jaro,
    jaro_winkler,
    sorensen_dice,
    StrSimError,
};

fn main() {
    let args = clap::Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!(", "))
        .about(clap::crate_description!())
        .arg(clap::Arg::new("hamming")
             .short('h')
             .long("hamming")
             .help("compute Hamming distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("levenshtein")
              .short('l')
              .long("levenshtein")
              .help("compute Levenshtein distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("norm_levenshtein")
              .short('L')
              .long("normalized-levenshtein")
              .help("compute normalized Levenshtein distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("osa")
              .short('o')
              .long("optimal-string-alignment")
              .help("compute optimal string alignment")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("damerau_levenshtein")
              .short('d')
              .long("damerau-levenshtein")
              .help("compute Damerau-Levenshtein distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("norm_damerau_levenshtein")
              .short('D')
              .long("normalized-damerau-levenshtein")
              .help("compute normalized Damerau-Levenshtein distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("jaro")
              .short('j')
              .long("jaro")
              .help("compute Jaro distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("jaro_winkler")
              .short('w')
              .long("jaro-winkler")
              .help("compute Jaro-Winkler distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("sorensen_dice")
              .short('s')
              .long("sorensen-dice")
              .help("compute Sørensen-Dice distance")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("all")
              .short('a')
              .long("all")
              .help("compute all distance metrics")
              .action(clap::ArgAction::SetTrue)
        ).arg(clap::Arg::new("words")
              .num_args(0..)
              .action(clap::ArgAction::Set)
              .value_name("WORD")
        ).get_matches();

    let stdin_words;
    let words : Vec<&str>;
    if let Some(v) = args.get_many::<&str>("words") {
        words = v.cloned().collect();
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut w = Vec::new();
        loop {
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(0) => break,

                Ok(_) => {
                    if input.ends_with('\n') {
                        input.pop();
                        if input.ends_with('\r') {
                            input.pop();
                        }
                    }
                    w.push(input);
                }

                Err(error) => unimplemented!("error: {}", error),
            }
        }

        stdin_words = w;
        words = stdin_words.iter().map(|w| w.as_str()).collect();
    }

    for (idx, w1) in words.iter().enumerate() {
        for w2 in &words[idx + 1..] {
            if args.get_flag("all") || args.get_flag("hamming") {
                match hamming(w1, w2) {
                    Ok(dist) => println!("Hamming distance of {} and {}: {}", w1, w2, dist),
                    Err(StrSimError::DifferentLengthArgs)
                        => println!("Hamming distance of {} and {} not possible", w1, w2),
                }
            }

            macro_rules! dist {
                ($arg:literal, $name:literal, $func:ident) => (
                    if args.get_flag("all") || args.get_flag($arg) {
                        println!(concat!($name, " distance of {} and {}: {}"), w1, w2, $func(w1, w2));
                    }
                );
            }

            dist!("levenshtein", "Levenshtein", levenshtein);
            dist!("norm_levenshtein", "Normalized Levenshtein", normalized_levenshtein);
            dist!("osa", "Optimal string alignment", osa_distance);
            dist!("damerau_levenshtein", "Damerau-Levenshtein", damerau_levenshtein);
            dist!("norm_damerau_levenshtein", "Normalized Damerau-Levenshtein",
                  normalized_damerau_levenshtein);
            dist!("jaro", "Jaro", jaro);
            dist!("jaro_winkler", "Jaro-Winkler", jaro_winkler);
            dist!("sorensen_dice", "Sørensen-Dice", sorensen_dice);
        }
    }
}
