use anyhow::Result;
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
const VOWEL: &[&str] = &[
    "AA", "AE", "AH", "AO", "AW", "AY", "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW",
];
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

pub fn output(v: &[String]) -> Result<()> {
    let out = std::io::stdout();
    let mut lock = out.lock();
    use std::io::Write;
    for s in v {
        writeln!(lock, "{}", s.to_lowercase())?;
    }
    Ok(())
}

fn findem_re(s: &str, re: &Regex) -> Vec<String> {
    re.captures_iter(&s).map(|cap| cap[1].to_string()).collect()
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
) -> Vec<String> {
    let phoneme_list = findwordphonemes(DATA, &word.to_uppercase());
    println!("{:?}", &phoneme_list);
    let phonemes = phoneme_list.get(0).unwrap().clone();
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
        let re = Regex::new(&pat).unwrap();
        res.push(re);
    }
    println!("regexes: {:?}", res);

    let mut result = vec![];
    DATA.lines().for_each(|l| {
        res.iter().try_for_each(|re| {
            let hits = findem_re(l, re);
            let found = !hits.is_empty();
            result.extend(hits);
            if found {
                Some(())
            } else {
                // Don't bother testing the other patterns, we already found a match.
                None
            }
        });
    });

    result
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
        let v = find_onepass(word, RhymeStyle::Syllabic, RhymeType::Rhyme, 2, true);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find_onepass_vowel() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::Vowel, RhymeType::Rhyme, 2, true);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find_onepass_conso() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::Consonant, RhymeType::Rhyme, 2, true);
        println!("{:?}", v);
    }
}
