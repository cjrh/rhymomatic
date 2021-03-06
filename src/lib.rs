use anyhow::{Context, Result};
use regex::Regex;
use structopt::clap::arg_enum;

//  Phoneme Example Translation
// ------- ------- -----------
// AA	odd     AA D
// AE	at	AE T
// AH	hut	HH AH T
// AO	ought	AO T
// AW	cow	K AW
// AY	hide	HH AY D
// B 	be	B IY
// CH	cheese	CH IY Z
// D 	dee	D IY
// DH	thee	DH IY
// EH	Ed	EH D
// ER	hurt	HH ER T
// EY	ate	EY T
// F 	fee	F IY
// G 	green	G R IY N
// HH	he	HH IY
// IH	it	IH T
// IY	eat	IY T
// JH	gee	JH IY
// K 	key	K IY
// L 	lee	L IY
// M 	me	M IY
// N 	knee	N IY
// NG	ping	P IH NG
// OW	oat	OW T
// OY	toy	T OY
// P 	pee	P IY
// R 	read	R IY D
// S 	sea	S IY
// SH	she	SH IY
// T 	tea	T IY
// TH	theta	TH EY T AH
// UH	hood	HH UH D
// UW	two	T UW
// V 	vee	V IY
// W 	we	W IY
// Y 	yield	Y IY L D
// Z 	zee	Z IY
// ZH	seizure	S IY ZH ER

const DATA: &str = include_str!("cmudict-0.7b.utf8");
// const VOWEL: &[&str] = &[
//     "AA", "AE", "AH", "AO", "AW", "AY", "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW",
// ];
// const PAT_TEMPLATE_SUFFIX: &str = r"(?m)^(\S*)  (.*{})$";
const PAT_TEMPLATE_SUFFIX: &str = r"(?m)^([a-zA-Z-]*)\S*  (.*{})$";
const PAT_TEMPLATE_PREFIX: &str = r"(?m)^(\S*)  ({}.*)$";
const PAT_TEMPLATE_ANY: &str = r"(?m)^(\S*)  (.*{}.*)$";

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum RhymeStyle {
        Syllabic,
        Vowel,
        Consonant,
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum RhymeType {
        Rhyme,
        Alliteration,
        Any,
    }
}

// This function is an optimized way of printing many
// lines to the screen, because it locks stdout and 
// doesn't flush the output buffer until the lock is
// released.
pub fn output(v: &[String]) -> Result<()> {
    let out = std::io::stdout();
    let mut lock = out.lock();
    use std::io::Write;
    for s in v {
        writeln!(lock, "{}", s.to_lowercase())?;
    }
    Ok(())
}

fn findem_re(s: &str, re: &Regex) -> Option<(String, String)> {
    re.captures(&s).map(|caps|
        (
            caps[1].to_string(),
            caps[2].to_string()
        )
    )
}

fn findwordphonemes(s: &str, word: &str) -> Vec<String> {
    let pat = format!(r"(?m)^{}  (.*)$", word);
    let re = Regex::new(&pat).unwrap();
    re.captures_iter(&s).map(|cap| cap[1].to_string()).collect()
}

pub fn find_onepass(
    word: &str,
    rhyme_style: RhymeStyle,
    rhyme_type: RhymeType,
    min_phonemes: usize,
    keep_emphasis: bool,
    bailout: Option<usize>,
) -> Result<Vec<String>> {
    let phoneme_list = findwordphonemes(DATA, &word.to_uppercase());
    println!("{:?}", &phoneme_list);
    // If we found the given word in the data, extract that,
    // otherwise return an error.
    let phonemes = phoneme_list
        .get(0)
        .context(format!("The word '{}' was not found in the database.", &word))?
        .clone();
    println!("{:?}", &phonemes);

    let match_phonemes = match rhyme_style {
        RhymeStyle::Syllabic => phonemes
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect(),
        RhymeStyle::Vowel => wild_consos(&phonemes, keep_emphasis),
        RhymeStyle::Consonant => wild_vowels(&phonemes),
    };
    // let match_phonemes = wild_consos(phonemes, true);
    println!("{:?}", &match_phonemes);

    let n = match_phonemes.len();
    let mut res = vec![];

    for i in (min_phonemes..n).rev() {
        let pat = match rhyme_type {
            RhymeType::Rhyme => {
                PAT_TEMPLATE_SUFFIX.replace("{}", &match_phonemes[n - i..].join(" "))
            }
            RhymeType::Alliteration => {
                PAT_TEMPLATE_PREFIX.replace("{}", &match_phonemes[..i].join(" "))
            }
            RhymeType::Any => {
                PAT_TEMPLATE_ANY.replace(
                    // TODO: this needs more thinking
                    "{}",
                    &match_phonemes[n - i..].join(" "),
                )
            }
        };
        // let pat = pat_template.replace("{}", &match_phonemes.join(" "));
        println!("{:?}", &pat);
        let re = Regex::new(&pat).context("Unexpected regex compile error")?;
        res.push(re);
    }
    println!("regexes:");
    res.iter().for_each(|r| println!("{:?}", &r));
    println!();
    // println!("regexes: {:?}", res);

    let mut result = vec![];
    DATA.lines().for_each(|l| {
        res.iter().try_for_each(|re| {
            match findem_re(l, re) {
                Some(hit) => {
                    result.push((hit.0, score(&phonemes, &hit.1)));
                    None
                },
                None => Some(())
            }
        });
    });

    result.sort_by_key(|tup| tup.1);
    Ok(result.iter().map(|tup| tup.0.clone()).collect())
}

fn score(word_phonemes: &str, candidate_phonemes: &str) -> u32 {
    // Rules: 
    // 10: same number of phonemes 
    // 9: same number of vowel phonemes
    // 8: off by 1 phoneme
    // 7: off by 2 phoneme
    let w = word_phonemes.split_ascii_whitespace().count() as i32;
    let c = candidate_phonemes.split_ascii_whitespace().count() as i32;
    (w - c).abs() as u32
}

fn wild_consos(phonemes: &str, keep_vowel_emph: bool) -> Vec<String> {
    let pat = r"([AEIOU][A-Z]*)(\d?)";
    let re = Regex::new(&pat).unwrap();
    phonemes
        .split_ascii_whitespace()
        .map(|s| {
            if let Some(caps) = re.captures(s) {
                let pre = caps.get(1).unwrap().as_str();
                let mut num = caps.get(2).unwrap().as_str();
                if !keep_vowel_emph {
                    num = r"\d?"
                }
                // println!("{}-{}", pre, num);
                format!(r"{}{}", pre, num)
            } else {
                r"\S*".to_string()
            }
        })
        .collect::<Vec<_>>()
}

fn wild_vowels(phonemes: &str) -> Vec<String> {
    let pat = r"([AEIOU][A-Z]*)(\d?)";
    let re = Regex::new(&pat).unwrap();
    phonemes
        .split_ascii_whitespace()
        .map(|s| {
            if let Some(caps) = re.captures(s) {
                let pre = caps.get(1).unwrap().as_str();
                let num = caps.get(2).unwrap().as_str();
                println!("{}-{}", pre, num);
                r"\S*".to_string()
            } else {
                s.to_string()
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let v = vec!["a", "b", "c"];
        let v = v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(output(&v).is_ok());
    }

    #[test]
    fn test_just_find_onepass() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::Syllabic, RhymeType::Rhyme, 2, true, None);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find_onepass_vowel() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::Vowel, RhymeType::Rhyme, 2, true, None);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find_onepass_conso() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::Consonant, RhymeType::Rhyme, 2, true, None);
        println!("{:?}", v);
    }
}
