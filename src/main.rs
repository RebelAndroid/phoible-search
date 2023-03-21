use std::collections::{HashMap, HashSet};



mod parser;
use parser::Parser;

#[derive(Debug)]
struct Language {
    name: String,
    phonemes: Vec<(String, Vec<String>)>,
}

fn main() {
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
                allophones_string.split(" ")
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
    let mut p = Parser::new("!t&d");
    println!("{:?}", p.expression());

}
