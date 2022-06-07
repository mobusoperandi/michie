use markdown::{tokenize, Block};
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

#[test]
// because markdown_toc does the wrong thing with some characters when generating links
fn limited_character_set_in_headings() {
    let readme = read_to_string("README.md").unwrap();
    let tokens = tokenize(&readme);
    tokens
        .into_iter()
        .filter_map(|block| match block {
            Block::Header(spans, _) => Some(spans),
            _ => None,
        })
        .for_each(|spans| {
            assert_eq!(spans.len(), 1);
            let span = spans.get(0).unwrap();
            let text = match span {
                markdown::Span::Text(text) => text,
                _ => panic!("heading contains something other than plain text"),
            };
            assert!(
                text.chars()
                    .all(|c| c.is_alphanumeric() || c == ' ' || c == '_' || c == '-'),
                "{}",
                text
            );
        });
}
