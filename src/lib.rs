#![allow(dead_code)]
extern crate yaml_rust;

use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::scanner::ScanError;

fn find_yaml_block(text: &str) -> Option<(usize, usize, usize)> {
    match text.starts_with("---\n") {
        true => {
            let slice_after_marker = &text[4..];
            let fm_end = slice_after_marker.find("---\n");
            if fm_end.is_none() {
                return None
            };

            let fm_end = fm_end.unwrap();
            Some((4,fm_end+4, fm_end+2*4))
        },
        false => None
    }
}

pub fn parse_and_find_content(text: &str) -> Result<(Option<Yaml>, &str), ScanError> {
    match find_yaml_block(text) {
        Some((fm_start, fm_end, content_start)) => {
            let yaml_str = &text[fm_start..fm_end];
            let mut documents = try!(YamlLoader::load_from_str(yaml_str));

            let rest_of_text = &text[content_start..];

            Ok((documents.pop(), rest_of_text))
        },
        None => Ok((None, text))
    }
}

pub fn parse(text: &str) -> Result<Option<Yaml>, ScanError> {
    let (matter, _) = try!(parse_and_find_content(text));
    Ok(matter)
}

#[test]
fn test_valid() {
    let test_string = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap();
    assert!(matter.is_some());
    let matter = matter.unwrap();
    assert!(matter.as_hash().is_some());
}

#[test]
fn test_valid_find_content() {
    let test_string = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_some());
    let matter = matter.unwrap();
    assert!(matter.as_hash().is_some());
    assert!(stripped_string.to_string() == test_string[31..].to_string());
}

#[test]
fn test_none() {
    let test_string = "something that's not yaml even if it has\n---\nsome: yaml\n--";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap();
    assert!(matter.is_none());
}

#[test]
fn test_none_find_content() {
    let test_string = "something that's not yaml even if it has\n---\nsome: yaml\n--";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_none());
    assert!(stripped_string.to_string() == test_string);

}


#[test]
fn test_empty() {
    let test_string = "---\n---\nsomething that's not yaml";

    println!("Original String{:?}", test_string);
    let matter = parse(test_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_ok());
    let matter = matter.unwrap();
    assert!(matter.is_none());
}

#[test]
fn test_empty_find_content() {
    let test_string = "---\n---\nsomething that's not yaml";

    println!("Original String{:?}", test_string);
    let result = parse_and_find_content(test_string);
    assert!(result.is_ok());
    let (matter, stripped_string) = result.unwrap();
    println!("Stripped String{:?}", stripped_string);
    println!("Matter: {:?}", matter);
    assert!(matter.is_none());
    assert!(stripped_string.to_string() == test_string[8..]);

}


#[test]
fn test_tests() {
    assert_eq!(2+2, 4);
}
