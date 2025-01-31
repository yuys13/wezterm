const VARIATION_SEQUENCES: &str = include_str!("../../data/emoji-variation-sequences.txt");

include!("../../src/emoji_presentation.rs");
include!("../../src/widechar_width.rs");

fn main() {
    println!("//! This file was generated by running:");
    println!("//! cd ../codegen ; cargo run > ../emoji_variation.rs");
    println!();
    emit_variation_map();
    emit_classify_table();
}

fn emit_classify_table() {
    let table = WcLookupTable::new();
    println!("use crate::widechar_width::{{WcLookupTable, WcWidth}};");
    println!();
    println!("pub const WCWIDTH_TABLE: WcLookupTable = WcLookupTable {{");
    println!("  table: [");

    for c in &table.table {
        println!("  WcWidth::{:?},", c);
    }

    println!("]}};");
}

/// Parses emoji-variation-sequences.txt, which is part of the UCD download
/// for a given version of the Unicode spec.
/// It defines which sequences can have explicit presentation selectors.
fn emit_variation_map() {
    let mut map = phf_codegen::Map::new();

    'next_line: for line in VARIATION_SEQUENCES.lines() {
        if let Some(lhs) = line.split('#').next() {
            if let Some(seq) = lhs.split(';').next() {
                let mut s = String::new();
                let mut last = None;
                for hex in seq.split_whitespace() {
                    match u32::from_str_radix(hex, 16) {
                        Ok(n) => {
                            let c = char::from_u32(n).unwrap();
                            s.push(c);
                            last.replace(c);
                        }
                        Err(_) => {
                            continue 'next_line;
                        }
                    }
                }

                if let Some(last) = last {
                    let first = if EMOJI_PRESENTATION.contains_u32(s.chars().next().unwrap() as u32) {
                        "Presentation::Emoji"
                    } else {
                        "Presentation::Text"
                    };
                    map.entry(
                        s,
                        &format!("({}, {})", first,
                        match last {
                            '\u{FE0F}' => "Presentation::Emoji",
                            '\u{FE0E}' => "Presentation::Text",
                            _ => unreachable!(),
                        }),
                    );
                }
            }
        }
    }

    println!("use crate::emoji::Presentation;");
    println!();
    println!(
        "pub static VARIATION_MAP: phf::Map<&'static str, (Presentation, Presentation)> = \n{};\n",
        map.build(),
    );
}
