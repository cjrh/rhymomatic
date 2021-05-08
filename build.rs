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

// use itertools::Itertools;
#[allow(dead_code)]
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use qp_trie::wrapper::BString;

macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}: {:?}", $context, timer.elapsed());
    };
}

fn serialize_trie_to_disk(
        qp: qp_trie::Trie<BString, String>,
        filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    time_it!("serializing qpt to bincode",
        let serialized = bincode::serialize(&qp)?
    );
    Ok(std::fs::write(filename, serialized)?)
}

fn load_trie_from_disk(
    filename: &str
) -> Result<qp_trie::Trie<BString, String>,
    Box<dyn std::error::Error>> {
    let bytes = std::fs::read(filename)?;
    time_it!("Deserialize",
        let result = bincode::deserialize(&bytes)?
    );
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running build!");

    let vowel_sounds = vec![
        "AA", "AE", "AH", "AO", "AW", "AY", "EH", "ER", "EY", "IH", "IY", "OW", "OY", "UH", "UW",
    ];
    // Set of the vowel sounds
    let vsset = vowel_sounds
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<String>>();

    // Reading the file
    let items: Vec<Vec<String>> = BufReader::new(File::open("src/cmudict-0.7b.utf8")?)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !(l.trim().is_empty()))
        .filter(|l| !l.starts_with(";;;"))
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        })
        .collect();

    // for x in &items[2000..2010] {
    //     println!("{:?}", x);
    // }

    // This map is the main one with words as keys and phoneme
    // as vector value. It will be used when a lookup word is
    // supplied from input. Once we have the phonemes for the
    // word, we can take some of the phonemes from the end and
    // do the lookups.
    let mut hmmain = HashMap::new();
    for x in &items[2000..2010] {
        let k = x[0].clone();
        let v = x[1..].to_vec();
        // println!("{:?} {:?}", &k, &v);
        hmmain.insert(k, v);
    }

    let re = regex::Regex::new(r"\d").unwrap();

    // This trie stores all the syllables as-is
    // SYLLABIC
    let mut qpt = qp_trie::Trie::new();
    // ASSONANCE
    let mut qpt_vowel_only = qp_trie::Trie::new();
    // CONSONANCE
    let mut qpt_conso_only = qp_trie::Trie::new();
    // We also need once of these for alliteration (first consonant)
    //let mut qpt_allit_only = qp_trie::Trie::new();

    // Sliding window to detect when
    for x in &items[5000..5100] {
        let phonemes = x[1..].iter().rev().map(|s| s.as_str()).collect::<Vec<_>>();

        // Remove emphasis
        let phonemes = phonemes
            .iter()
            .map(|ph| re.replace_all(ph, "").into())
            .collect::<Vec<String>>();
        qpt.insert_str(phonemes.join("-").as_str(), x[0].clone());

        let phonemes_vowel = phonemes
            .iter()
            .filter(|&p| vsset.contains(p))
            .map(|s| &**s)
            .collect::<Vec<_>>();
        qpt_vowel_only.insert_str(phonemes_vowel.join("-").as_str(), x[0].clone());

        let phonemes_conso = phonemes
            .iter()
            .filter(|&p| !vsset.contains(p))
            .map(|s| &**s)
            .collect::<Vec<_>>();
        qpt_conso_only.insert_str(phonemes_conso.join("-").as_str(), x[0].clone());
    }

    println!("trie");
    println!("{:?}", &qpt);

    println!();
    println!("{:?}", qpt.subtrie_str("IY-L-T"));
    println!();
    println!("{:?}", qpt.subtrie_str("IY"));
    println!();
    println!("{:?}", qpt.subtrie_str("Z-N-AH"));

    time_it!("serializing qpt to bincode",
        let serialized = bincode::serialize(&qpt)?
    );
    // println!("{:?}", &serialized);
    let mut writer = std::io::BufWriter::new(std::fs::File::create("./qpt.bin")?);
    time_it!("write bincode", {
        writer.write_all(&serialized)?;
    });

    serialize_trie_to_disk(qpt, "./qpt2.bin")?;
    let qp_from_disk = load_trie_from_disk("./qpt2.bin")?;
    println!("{:?}", qp_from_disk.subtrie_str("IY-L-T"));


    println!();
    println!();
    println!();
    println!();
    let xyz = vec!["a", "b"];

    println!("{:?}", qpt_conso_only.subtrie_str("N-SH"));
    println!();
    println!("{:?}", qpt_vowel_only.subtrie_str("AA-AH"));
    // println!("");
    // println!("{:?}", qpt.subtrie_str("Z-N-AH"));

    println!();
    println!();

    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    write!(
        &mut file,
        "static KEYWORDS: phf::Map<&'static str, Keyword> = "
    )
    .unwrap();
    println!("Wrote to file.");
    let mut m = phf_codegen::Map::new();
    let s = m
        .entry("loop", "Keyword::Loop")
        .entry("continue", "Keyword::Continue")
        .entry("break", "Keyword::Break")
        .entry("fn", "Keyword::Fn")
        .entry("extern", "Keyword::Extern")
        .build();
    writeln!(&mut file, "{}", s).unwrap();
    writeln!(&mut file, ";").unwrap();
    Ok(())
}
