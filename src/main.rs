use rhymomatic::{RhymeStyle, RhymeType, find_onepass};
// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
#[derive(structopt::StructOpt)]
struct Args {
    #[structopt(short = "w", long = "word")]
    word: String,
    #[structopt(short = "s", long = "style", default_value = "syllabic")]
    rhyme_style: RhymeStyle,
    #[structopt(short = "t", long = "type", default_value = "rhyme")]
    rhyme_type: RhymeType,
}


#[paw::main]
fn main(args: Args) {
    let results = find_onepass(
        &args.word,
        args.rhyme_style,
        args.rhyme_type,
        2,
        true,
    );

    rhymomatic::output(&results);

    // if let Err(e) = rhymomatic::get_rhymes(args.word) {
    //     eprintln!("Error: {}", &e);
    //     std::process::exit(1)
    // }
}
