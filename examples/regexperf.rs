use std::fs::File;
use regex::Regex;
use std::io::Read;

macro_rules! time_it {
    ($context:literal, $s:stmt) => {
        let timer = std::time::Instant::now();
        $s
        println!("{}: {:?}", $context, timer.elapsed());
    };
}

fn main() {
    println!("reperf");
    let mut s = String::new();
    File::open("src/cmudict-0.7b.utf8")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();
    println!("{}", &s[..20]);
    let target = "AW1 T";
    let timer = std::time::Instant::now();
    findem(&s, target);
    println!("time: {:?}", timer.elapsed());
}

fn findem(s: &str, target: &str) {
    let pat = format!(r"(?m)^(\S*)  (.*{}.*)$", target);
    let out = std::io::stdout();
    let mut lock = out.lock();
    println!("{}", &pat);
    let re = Regex::new(&pat).unwrap();
    use std::io::{Write};
    for cap in re.captures_iter(&s) {
        // println!("{} {}", &cap[2], &cap[1]);
        writeln!(lock, "{} {}", &cap[2], &cap[1]);
    };
}