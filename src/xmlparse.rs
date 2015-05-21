extern crate xml;

use std::fs::File;

use xml::reader::EventReader;
use xml::reader::events::*;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {
    let file = File::open("src/sample_2.xml").unwrap();

    let mut parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, .. } => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            XmlEvent::EndElement { name } => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
