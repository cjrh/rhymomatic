include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub fn make_rhymes() -> Vec<&'static str> {
    include_str!("cmudict-0.7b.utf8").lines().collect()
}

fn main() {
    println!("Hello, world!");
    let v = make_rhymes();
    println!(
        "{:?}",
        v.iter().take(3).map(|s| s.to_string()).collect::<Vec<_>>()
    );
    println!();
    let f = std::fs::File::create("blah.txt");
}
