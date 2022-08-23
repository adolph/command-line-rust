use clap::{App, Arg};

fn main() {
    // println!("Hello, world!");
    // println!("{:?}",std::env::args());
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Adolph Trudeau")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("text")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");


    // println!("{:#?}",matches);
    // println!("{:#?}",text.join(" "));
    // println!("{:#?}",omit_newline);
    let ending = if omit_newline {""} else {"\n"};
    print!("{}{}",text.join(" "),ending);
}
