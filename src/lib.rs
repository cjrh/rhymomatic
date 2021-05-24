use regex::Regex;

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
        $block
        println!("{}: {:?}", $context, timer.elapsed());
    };
}

const DATA: &str = include_str!("cmudict-0.7b.utf8");
const VOWEL: &'static [&'static str] = &[
    "AA", "AE", "AH", "AO", "AW", "AY", "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW",
];

pub fn get_rhymes(word: String) -> Result<()> {
    time_it!("two printlines",{
    println!("123");
    println!("456");
    });
    println!("{}", &DATA[..20]);
    Ok(())
}

fn output(v: &Vec<String>) {
    let out = std::io::stdout();
    let mut lock = out.lock();
    use std::io::{Write};
    for s in v {
        writeln!(lock, "{}", s).unwrap();
    }
}

fn findem(s: &str, target: &str, pat: &str) -> Vec<String> {
    let re = Regex::new(&pat).unwrap();
    re.captures_iter(&s)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn findwordphonemes(s: &str, word: &str) -> Vec<String> {
    let pat = format!(r"(?m)^{}  (.*)$", word);
    let re = Regex::new(&pat).unwrap();
    re.captures_iter(&s)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn find_any(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  (.*{}.*)$", target);
    findem(DATA, target, &pat)
}

fn find_suffix(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
    findem(DATA, target, &pat)
}

fn find_suffix_vowel(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the consonant parts with wildcards
    let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
    findem(DATA, target, &pat)
}

fn find_suffix_conso(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the vowel parts with wildcards
    let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
    findem(DATA, target, &pat)
}

fn find_prefix(target: &str) -> Vec<String> {
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, target, &pat)
}

fn find_prefix_vowel(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the consonant parts with wildcards
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, target, &pat)
}

fn find_prefix_conso(target: &str) -> Vec<String> {
    // TODO: target must be modified to replace the vowel parts with wildcards
    let pat = format!(r"(?m)^(\S*)  ({}.*)$", target);
    findem(DATA, target, &pat)
}

fn wild_vowels(phonemes: &str) -> String {
    phonemes
        .split_ascii_whitespace()
        .map(|s| {
            if VOWEL.contains(&s) {
                r"\S*"
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn wild_consos(phonemes: &str) -> String {
    phonemes
        .split_ascii_whitespace()
        .map(|s| {
            if !VOWEL.contains(&s) {
                r"\S*"
            } else {
                s
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
        println!("{:?}", phonemes);
        assert_eq!(phonemes, vec!["F OW1 K AH0 S"]);
        });
    }

    #[test]
    fn test_findem_any() {
        let target = r"D AY\S S";
        let pat = format!(r"(?m)^(\S*)  (.*{}.*)$", target);
        let v = findem(DATA, target, &pat);
        output(&v);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_findem_end() {
        let target = "AW1 T";
        let pat = format!(r"(?m)^(\S*)  (.*{})$", target);
        let v = findem(DATA, target, &pat);
        output(&v);
        assert_eq!(1, 1);
    }

    #[test]
    fn test_findem_all() {
        let phonemes = "F OW1 K AH0 S";  // FOCUS
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
        let phonemes = "F OW1 K AH0 S";  // FOCUS
        let wild = wild_vowels(phonemes);
        assert_eq!(wild, r"F \S* K \S* S");
    }
}