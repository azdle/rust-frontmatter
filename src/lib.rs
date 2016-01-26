#![allow(dead_code)]
extern crate yaml_rust;

mod frontmatter {
    use yaml_rust::{Yaml, YamlLoader};
    use yaml_rust::scanner::ScanError;

    pub fn parse(text: &str) -> Result<Option<Yaml>, ScanError> {
        match text.starts_with("---\n") {
            true => {
                let slice_after_marker = &text[4..];
                let fm_end = slice_after_marker.find("---");
                if fm_end.is_none() {
                    return Ok(None)
                };

                let fm_end = fm_end.unwrap();
                let yaml_str = &text[4..4+fm_end];
                let mut documents = try!(YamlLoader::load_from_str(yaml_str));

                Ok(documents.pop())
            },
            false => Ok(None)
        }
    }

    #[test]
    fn test_valid() {
        let test_string = "---\ntitle: Valid Yaml Test\n---\nsomething that's not yaml";

        let matter = parse(test_string);
        println!("Original String{:?}", test_string);
        println!("Matter: {:?}", matter);
        assert!(matter.is_ok());
        let matter = matter.unwrap();
        assert!(matter.is_some());
        let matter = matter.unwrap();
        assert!(matter.as_hash().is_some());
    }

    #[test]
    fn test_none() {
        let test_string = "something that's not yaml even if it has\n---\nsome: yaml\n--";

        let matter = parse(test_string);
        println!("Original String{:?}", test_string);
        println!("Matter: {:?}", matter);
        assert!(matter.is_ok());
        let matter = matter.unwrap();
        assert!(matter.is_none());
    }


    #[test]
    fn test_empty() {
        let test_string = "---\n---\nsomething that's not yaml";

        let matter = parse(test_string);
        println!("Original String{:?}", test_string);
        println!("Matter: {:?}", matter);
        assert!(matter.is_ok());
        let matter = matter.unwrap();
        assert!(matter.is_none());
    }
}


#[test]
fn test_tests() {
    assert_eq!(2+2, 4);
}
