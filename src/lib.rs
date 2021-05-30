use regex::Regex;
use std::fmt;
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
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! time_it {
    ($context:literal, $block:block) => {
        let timer = std::time::Instant::now();
        let tmp = $block;
        println!("{}: {:?}", $context, timer.elapsed());
        tmp
    };
}

const DATA: &str = include_str!("cmudict-0.7b.utf8");
const VOWEL: &'static [&'static str] = &[
    "AA", "AE", "AH", "AO", "AW", "AY", "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW",
];
const PAT_TEMPLATE_SUFFIX: &str = r"(?m)^(\S*)  (.*{})$";
const PAT_TEMPLATE_PREFIX: &str = r"(?m)^(\S*)  ({}.*)$";
const PAT_TEMPLATE_ANY: &str = r"(?m)^(\S*)  (.*{}.*)$";

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum RhymeStyle {
        SYLLABIC,
        VOWEL,
        CONSONANT,
    }
}

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum RhymeType {
        RHYME,
        ALLITERATION,
        ANY,
    }
}

pub fn get_rhymes(word: String) -> Result<()> {
    println!("{}", &DATA[..20]);
    Ok(())
}

pub fn output(v: &Vec<String>) {
    let out = std::io::stdout();
    let mut lock = out.lock();
    use std::io::Write;
    for s in v {
        writeln!(lock, "{}", s.to_lowercase()).unwrap();
    }
}

fn findem(s: &str, pat: &str) -> Vec<String> {
    let re = Regex::new(&pat).unwrap();
    re.captures_iter(&s).map(|cap| cap[1].to_string()).collect()
}

fn findem_re(s: &str, re: &Regex) -> Vec<String> {
    re.captures_iter(&s).map(|cap| cap[1].to_string()).collect()
}

fn findwordphonemes(s: &str, word: &str) -> Vec<String> {
    let pat = format!(r"(?m)^{}  (.*)$", word);
    let re = Regex::new(&pat).unwrap();
    re.captures_iter(&s).map(|cap| cap[1].to_string()).collect()
}

fn find_any(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  (.*{}.*)$", target);
    findem(DATA, &pat)
}

fn find_suffix(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
    findem(DATA, &pat)
}

fn find_suffix_vowel(target: &str) -> Vec<String> {
    let phoneme_list = findwordphonemes(DATA, target);
    println!("{:?}", &phoneme_list);
    let phonemes = phoneme_list.get(0).unwrap();
    println!("{:?}", &phonemes);
    let match_phonemes = wild_consos(phonemes, true);
    println!("{:?}", &match_phonemes);
    let pat = format!(r"(?m)^(\S*)  (.*{})$", match_phonemes.join(" "));
    println!("{:?}", &pat);
    findem(DATA, &pat)
}

fn find(
    word: &str,
    rhyme_style: RhymeStyle,
    rhyme_type: RhymeType,
    min_phonemes: usize,
    keep_emphasis: bool,
) -> Vec<String> {
    let phoneme_list = findwordphonemes(DATA, word);
    println!("{:?}", &phoneme_list);
    let phonemes = phoneme_list.get(0).unwrap().clone();
    println!("{:?}", &phonemes);

    let match_phonemes = match rhyme_style {
        RhymeStyle::SYLLABIC => phonemes
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect(),
        RhymeStyle::VOWEL => wild_consos(&phonemes, keep_emphasis),
        RhymeStyle::CONSONANT => wild_vowels(&phonemes),
    };
    // let match_phonemes = wild_consos(phonemes, true);
    println!("{:?}", &match_phonemes);

    let mut result = vec![];
    let n = match_phonemes.len();
    for i in (min_phonemes..n).rev() {
        let pat = match rhyme_type {
            RhymeType::RHYME => {
                PAT_TEMPLATE_SUFFIX.replace("{}", &match_phonemes[n - i..].join(" "))
            }
            RhymeType::ALLITERATION => {
                PAT_TEMPLATE_PREFIX.replace("{}", &match_phonemes[..i].join(" "))
            }
            RhymeType::ANY => {
                PAT_TEMPLATE_ANY.replace(
                    // TODO: this needs more thinking
                    "{}",
                    &match_phonemes[n - i..].join(" "),
                )
            }
        };
        // let pat = pat_template.replace("{}", &match_phonemes.join(" "));
        // println!("{:?}", &pat);
        result.extend(findem(DATA, &pat));
    }
    result
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
        RhymeStyle::SYLLABIC => phonemes
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect(),
        RhymeStyle::VOWEL => wild_consos(&phonemes, keep_emphasis),
        RhymeStyle::CONSONANT => wild_vowels(&phonemes),
    };
    // let match_phonemes = wild_consos(phonemes, true);
    println!("{:?}", &match_phonemes);

    let n = match_phonemes.len();
    let mut res = vec![];

    for i in (min_phonemes..n).rev() {
        let pat = match rhyme_type {
            RhymeType::RHYME => {
                PAT_TEMPLATE_SUFFIX.replace("{}", &match_phonemes[n - i..].join(" "))
            }
            RhymeType::ALLITERATION => {
                PAT_TEMPLATE_PREFIX.replace("{}", &match_phonemes[..i].join(" "))
            }
            RhymeType::ANY => {
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
        res.iter().for_each(|re| {
            let hits = findem_re(l, re);
            let found = hits.len() > 0;
            result.extend(hits);
            if found {
                // Don't bother testing the other patterns, we already found a match.
                return;
            }
        })
    });

    // result.extend(findem(DATA, &pat));

    result
}

fn find_suffix_conso(target: &str) -> Vec<String> {
    let phoneme_list = findwordphonemes(DATA, target);
    println!("{:?}", &phoneme_list);
    let phonemes = phoneme_list.get(0).unwrap();
    println!("{:?}", &phonemes);
    let match_phonemes = wild_vowels(phonemes);
    println!("{:?}", &match_phonemes);
    let pat = format!(r"(?m)^(\S*)  (.*{})$", match_phonemes.join(" "));
    println!("{:?}", &pat);
    findem(DATA, &pat)
}

fn find_prefix(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, &pat)
}

fn find_prefix_vowel(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the consonant parts with wildcards
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, &pat)
}

fn find_prefix_conso(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the vowel parts with wildcards
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, &pat)
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
                println!("{}-{}", pre, num);
                format!(r"{}{}", pre, num)
            } else {
                r"\S*".to_string()
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
        let v = v.iter().map(|s| s.to_string()).collect();
        output(&v);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_find_word() {
        let word = "FOCUS";
        time_it!("find word", {
            let phonemes = findwordphonemes(DATA, word);
            // println!("{:?}", phonemes);
            assert_eq!(phonemes, vec!["F OW1 K AH0 S"]);
        });
    }

    #[test]
    fn test_findem_any() {
        let target = r"D AY\S S";
        let pat = format!(r"(?m)^(\S*)  (.*{}.*)$", target);
        let v = findem(DATA, &pat);
        output(&v);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_findem_end() {
        let target = "AW1 T";
        let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
        let v = findem(DATA, &pat);
        output(&v);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_findem_all() {
        let phonemes = "F OW1 K AH0 S"; // FOCUS
        let vphonemes = phonemes.split_ascii_whitespace().collect::<Vec<_>>();
        for i in 0..vphonemes.len() - 2 {
            let target = vphonemes[i..].join(" ");
            println!("For target: {}", &target);
            let hits = find_suffix(&target);
            output(&hits);
            println!("\n");
        }
    }

    #[test]
    fn test_wild_vowel() {
        let phonemes = "F OW1 K AH0 S"; // FOCUS
        let wild = wild_vowels(phonemes).join(" ");
        assert_eq!(wild, r"F \S* K \S* S");
    }

    #[test]
    fn test_wild_conso_emph() {
        let phonemes = "F OW1 K AH0 S"; // FOCUS
        let wild = wild_consos(phonemes, true).join(" ");
        assert_eq!(wild, r"\S* OW1 \S* AH0 \S*");
    }

    #[test]
    fn test_wild_conso_no_emph() {
        let phonemes = "F OW1 K AH0 S"; // FOCUS
        let wild = wild_consos(phonemes, false).join(" ");
        assert_eq!(wild, r"\S* OW\d? \S* AH\d? \S*");
    }

    #[test]
    fn test_find_suffix_vowel() {
        let word = "FOCUS";
        let v = find_suffix_vowel(word);
        println!("{:?}", v);
    }

    #[test]
    fn test_find_suffix_conso() {
        let word = "FOCUS";
        let v = find_suffix_conso(word);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find1() {
        let word = "FOCUS";
        let v = find(word, RhymeStyle::SYLLABIC, RhymeType::RHYME, 3, true);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find2() {
        let word = "FOCUS";
        let v = find(word, RhymeStyle::VOWEL, RhymeType::RHYME, 4, true);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find3() {
        let word = "FOCUS";
        let v = find(word, RhymeStyle::CONSONANT, RhymeType::RHYME, 4, true);
        println!("{:?}", v);
    }

    #[test]
    fn test_just_find_onepass() {
        let word = "FOCUS";
        let v = find_onepass(word, RhymeStyle::SYLLABIC, RhymeType::RHYME, 2, true);
        println!("{:?}", v);
    }
}
