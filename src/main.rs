use anyhow::Result;
use clap::Parser;
use rhymomatic::{find_onepass, RhymeStyle, RhymeType};
// include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Provide the word to find rhymes for.
    #[arg(short = 'w', long = "word")]
    word: String,
    /// The style of rhyming. "syllabic" means to match both
    /// vowel and consonant sounds. "vowel" means to match
    /// only vowel sounds with consonants allowed to not
    /// match those in the given word. "consonant" is the
    /// opposite: only consonants in the given word will be
    /// matched, with vowels being allowed to be different.
    #[arg(short = 's', long = "style", value_enum, default_value = "syllabic")]
    rhyme_style: RhymeStyle,
    /// The type of rhyme. "rhyme" means to try to match the
    /// given word from the end, like "POCUS" and "FOCUS".
    /// Alternatively you can give "alliteration", which
    /// will start matching from the front of the given
    /// word, like "POCUS" and "POCKET". Finally, you can
    /// provide "any", which means that phonemes in the
    /// given word will be allowed to match anywhere.
    #[arg(short = 't', long = "type", value_enum, default_value = "rhyme")]
    rhyme_type: RhymeType,
    /// The minimum number of phonemes to match. The lower this is,
    /// the more matching words will be found, but the strength of
    /// the rhyme gets weaker. For example, with a min length of
    /// 1, the words "SANDALS" and "HIPPOS" will be matched because
    /// they share a single matching phoneme in the trailing "S"
    /// sound. Usually this is not what you want. A min length
    /// of 2-3 is recommended.
    #[arg(short = 'm', long = "minphonemes", default_value_t = 2)]
    min_phonemes: usize,
    /// This setting will disable the requirement to match the
    /// emphasis in the given word.
    #[arg(short = 'n', long = "noemph")]
    noemph: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let results = find_onepass(
        &args.word,
        args.rhyme_style,
        args.rhyme_type,
        args.min_phonemes,
        !args.noemph,
        None,
    )?;
    rhymomatic::output(&results)?;
    Ok(())
}
