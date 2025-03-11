
const BIO: &str = include_str!("../bio.txt");

pub fn bio() -> impl Iterator<Item = String> {
    BIO.lines().map(|s| s.to_string() + "\n")
}
