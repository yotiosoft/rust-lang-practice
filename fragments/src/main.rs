use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn show(table: &Table) {
    for (artist, works) in table {          // artist: &String, works: &Vec<String>
        println!("works by {}:", artist);
        for work in works {                 // work: &String
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(), vec!["many madrigals".to_string(), "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(), vec!["The Musicians".to_string(), "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(), vec!["Perseus with the head of Medusa".to_string(), "a salt celler".to_string()]);

    sort_works(&mut table);     // 可変参照（sort_worksはtableを変更できる）

    show(&table);               // 共有参照（showはtableを変更できない）
}
