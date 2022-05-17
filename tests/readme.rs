use regex::Regex;
use std::{fs::read_to_string, path::Path};
#[test]
fn ci_badge() {
    let readme = read_to_string("README.md").unwrap();
    let regex = Regex::new("(?m)/actions/workflows/(.*?)(?:/|$)").unwrap();
    let filename = regex
        .captures_iter(&readme)
        .map(|captures| captures.get(1).expect("a capture").as_str())
        .reduce(|prev, curr| {
            assert_eq!(prev, curr);
            prev
        })
        .expect("at least one match");
    let path = format!(".github/workflows/{filename}");
    assert!(Path::new(&path).exists());
}
