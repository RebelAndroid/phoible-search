use std::collections::{HashMap};
mod parser;
mod interpreter;
use parser::{Parser};

use crate::interpreter::has_phonemes;

#[derive(Debug)]
pub struct Language {
    name: String,
    phonemes: Vec<(String, Vec<String>)>,
}

#[derive(clap_derive::Parser, Debug)]
struct Args{
    input_expression: String,
}

fn main() {
    let args = {
        use clap::Parser;
        Args::parse()
    };
    let mut reader =
        csv::Reader::from_path("phoible_data.csv").expect("unable to read phoible data!");
    let header = reader.headers().unwrap();
    let id_column = header
        .iter()
        .position(|item| item == "InventoryID")
        .unwrap();
    let name_column = header
        .iter()
        .position(|item| item == "LanguageName")
        .unwrap();
    let phoneme_column = header.iter().position(|item| item == "Phoneme").unwrap();
    let allophone_column = header.iter().position(|item| item == "Allophones").unwrap();

    let mut languages: HashMap<u32, Language> = HashMap::new();

    for record in reader.records() {
        let r = record.unwrap();

        let id = r.get(id_column).unwrap().parse::<u32>().unwrap();
        let name = r.get(name_column).unwrap().to_string();
        let phoneme = r.get(phoneme_column).unwrap().to_string();
        let allophones_string = r
            .get(allophone_column)
            .unwrap();
            let allophones = if allophones_string != "NA"{
                allophones_string.split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
            }else{
                vec![]
            };
            

        match languages.get_mut(&id) {
            Some(language) => language.phonemes.push((phoneme, allophones)),
            None => {
                languages.insert(
                    id,
                    Language {
                        name,
                        phonemes: vec![(phoneme, allophones)],
                    },
                );
            }
        }
    }
    let mut p = Parser::new(&args.input_expression);
    let ex = *p.expression();
    let mut count = 0;
    println!("{:#?}", ex);
    for (i, language) in &languages{
        if has_phonemes(language, &ex){
            println!("[{}]{}", i, language.name);
            count += 1;
        }
    }
    println!("{} languages out of {} satisfy {}", count, languages.len(), args.input_expression);

}
