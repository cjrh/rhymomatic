use qp_trie::wrapper::BString;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
type RhymeTrie = qp_trie::Trie<BString, String>;

macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}: {:?}", $context, timer.elapsed());
    };
}

struct Tries {
    allit: RhymeTrie,
    allit_vowel: RhymeTrie,
    allit_conso: RhymeTrie,
    syllabic: RhymeTrie,
    vowel: RhymeTrie,
    conso: RhymeTrie,
}

impl Tries {
    fn new() -> Tries {
        let bqpt_allit = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_allit.bin"));
        let bqpt_allit_vowel = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_allit_vowel.bin"));
        let bqpt_allit_conso = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_allit_conso.bin"));
        let bqpt_syllabic = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_syllabic.bin"));
        let bqpt_vowel = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_vowel_only.bin"));
        let bqpt_conso = include_bytes!(concat!(env!("OUT_DIR"), "/qpt_conso_only.bin"));

        Tries {
            allit: bincode::deserialize(bqpt_allit).unwrap(),
            allit_vowel: bincode::deserialize(bqpt_allit_vowel).unwrap(),
            allit_conso: bincode::deserialize(bqpt_allit_conso).unwrap(),
            syllabic: bincode::deserialize(bqpt_syllabic).unwrap(),
            vowel: bincode::deserialize(bqpt_vowel).unwrap(),
            conso: bincode::deserialize(bqpt_conso).unwrap(),
        }
    }
}

pub fn get_rhymes(word: String) -> Result<()> {
    // help for hashmap serialization: https://github.com/bincode-org/bincode/issues/230
    time_it!("Load all tries",
        let tries = Tries::new()
    );
    println!("{:?}", tries.syllabic.subtrie_str("IY-L-T"));
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}