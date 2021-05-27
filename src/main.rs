use rhymomatic::{RhymeStyle, RhymeType, find_onepass};
// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
#[derive(structopt::StructOpt)]
struct Args {
    /// Provide the word to find rhymes for.
    #[structopt(short = "w", long = "word")]
    word: String,
    /// The style of rhyming. "syllabic" means to match both
    /// vowel and consonant sounds. "vowel" means to match
    /// only vowel sounds with consonants allowed to not
    /// match those in the given word. "consonant" is the
    /// opposite: only consonants in the given word will be
    /// matched, with vowels being allowed to be different.
    #[structopt(short = "s", long = "style", default_value = "syllabic")]
    rhyme_style: RhymeStyle,
    /// The type of rhyme. "rhyme" means to try to match the
    /// given word from the end, like "POCUS" and "FOCUS".
    /// Alternatively you can give "alliteration", which
    /// will start matching from the front of the given
    /// word, like "POCUS" and "POCKET". Finally, you can
    /// provide "any", which means that phonemes in the
    /// given word will be allowed to match anywhere.
    #[structopt(short = "t", long = "type", default_value = "rhyme")]
    rhyme_type: RhymeType,
    /// The minimum number of phonemes to match. The lower this is,
    /// the more matching words will be found, but the strength of
    /// the rhyme gets weaker. For example, with a min length of
    /// 1, the words "SANDALS" and "HIPPOS" will be matched because
    /// they share a single matching phoneme in the trailing "S"
    /// sound. Usually this is not what you want. A min length
    /// of 2-3 is recommended.
    #[structopt(short = "m", long = "minphonemes", default_value = "2")]
    min_phonemes: usize,
    /// This setting will disable the requirement to match the
    /// emphasis in the given word.
    #[structopt(short = "e", long = "noemph")]
    noemph: bool,
}


#[paw::main]
fn main(args: Args) {
    let results = find_onepass(
        &args.word,
        args.rhyme_style,
        args.rhyme_type,
        args.min_phonemes,
        !args.noemph,
    );

    rhymomatic::output(&results);

    // if let Err(e) = rhymomatic::get_rhymes(args.word) {
    //     eprintln!("Error: {}", &e);
    //     std::process::exit(1)
    // }
}
