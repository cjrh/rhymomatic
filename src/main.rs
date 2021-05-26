// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn make_rhymes() -> Vec<&'static str> {
    include_str!("cmudict-0.7b.utf8").lines().collect()
}

#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short = "w", long = "word")]
    word: String,
    #[structopt(short = "s", long = "style", default_value = "syllabic")]
    rhyme_style: rhymomatic::RhymeStyle,
}


#[paw::main]
fn main(args: Args) {
    println!("Hello, world!");
    let v = make_rhymes();
    println!(
        "{:?}",
        v.iter().take(3).map(|s| s.to_string()).collect::<Vec<_>>()
    );
    println!();
    if let Err(e) = rhymomatic::get_rhymes(args.word) {
        eprintln!("Error: {}", &e);
        std::process::exit(1)
    }
}
