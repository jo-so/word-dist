use clap::{
    crate_authors,
    crate_description,
    crate_name,
    crate_version,
    App,
    Arg,
};
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
    let args = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!(", "))
        .about(crate_description!())
        .arg(Arg::new("hamming")
             .short('h')
             .long("hamming")
             .about("compute Hamming distance")
        ).arg(Arg::new("levenshtein")
              .short('l')
              .long("levenshtein")
              .about("compute Levenshtein distance")
        ).arg(Arg::new("norm_levenshtein")
              .short('L')
              .long("normalized-levenshtein")
              .about("compute normalized Levenshtein distance")
        ).arg(Arg::new("osa")
              .short('o')
              .long("optimal-string-alignment")
              .about("compute optimal string alignment")
        ).arg(Arg::new("damerau_levenshtein")
              .short('d')
              .long("damerau-levenshtein")
              .about("compute Damerau-Levenshtein distance")
        ).arg(Arg::new("norm_damerau_levenshtein")
              .short('D')
              .long("normalized-damerau-levenshtein")
              .about("compute normalized Damerau-Levenshtein distance")
        ).arg(Arg::new("jaro")
              .short('j')
              .long("jaro")
              .about("compute Jaro distance")
        ).arg(Arg::new("jaro_winkler")
              .short('w')
              .long("jaro-winkler")
              .about("compute Jaro-Winkler distance")
        ).arg(Arg::new("sorensen_dice")
              .short('s')
              .long("sorensen-dice")
              .about("compute Sørensen-Dice distance")
        ).arg(Arg::new("all")
              .short('a')
              .long("all")
              .about("compute all distance metrics")
        ).arg(Arg::new("words")
              .multiple_values(true)
              .multiple_occurrences(true)
              .value_name("WORD")
        ).get_matches();

    let stdin_words;
    let words : Vec<&str>;
    if let Some(v) = args.values_of("words") {
        words = v.collect();
    } else {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut w = Vec::new();
        loop {
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(n) if n == 0 => break,

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
            if args.is_present("all") || args.is_present("hamming") {
                match hamming(w1, w2) {
                    Ok(dist) => println!("Hamming distance of {} and {}: {}", w1, w2, dist),
                    Err(StrSimError::DifferentLengthArgs)
                        => println!("Hamming distance of {} and {} not possible", w1, w2),
                }
            }

            macro_rules! dist {
                ($args:ident, $arg:literal, $name:literal, $func:ident, $w1:ident, $w2:ident) => (
                    if $args.is_present("all") || $args.is_present($arg) {
                        println!(concat!($name, " distance of {} and {}: {}"), $w1, $w2, $func($w1, $w2));
                    }
                );
            }

            dist!(args, "levenshtein", "Levenshtein", levenshtein, w1, w2);
            dist!(args, "norm_levenshtein", "Normalized Levenshtein",
                  normalized_levenshtein, w1, w2);
            dist!(args, "osa", "Optimal string alignment", osa_distance, w1, w2);
            dist!(args, "damerau_levenshtein", "Damerau-Levenshtein",
                  damerau_levenshtein, w1, w2);
            dist!(args, "norm_damerau_levenshtein", "Normalized Damerau-Levenshtein",
                  normalized_damerau_levenshtein, w1, w2);
            dist!(args, "jaro", "Jaro", jaro, w1, w2);
            dist!(args, "jaro_winkler", "Jaro-Winkler", jaro_winkler, w1, w2);
            dist!(args, "sorensen_dice", "Sørensen-Dice", sorensen_dice, w1, w2);
        }
    }
}
