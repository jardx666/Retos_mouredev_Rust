use std::{io, array};
use std::collections::HashMap;


fn main() {

    let mut input = String::new();
    let mut hackerconvert = HashMap::new();
    hackerconvert.insert('a', '4');
    hackerconvert.insert('b', '8');
    hackerconvert.insert('c', '[');
    hackerconvert.insert('d', ')');
    hackerconvert.insert('e', '3');
    hackerconvert.insert('f', 'v');
    hackerconvert.insert('g', '&');
    hackerconvert.insert('h', '#');
    hackerconvert.insert('i', '!');
    hackerconvert.insert('j', ']');
    hackerconvert.insert('k', '>');
    hackerconvert.insert('l', '1');
    hackerconvert.insert('m', '^');
    hackerconvert.insert('n', '|');
    hackerconvert.insert('o', 'q');
    hackerconvert.insert('p', '*');
    hackerconvert.insert('q', '0');
    hackerconvert.insert('r', '`');
    hackerconvert.insert('s', '5');
    hackerconvert.insert('t', '7');
    hackerconvert.insert('u', 'v');
    hackerconvert.insert('v', '/');
    hackerconvert.insert('w', ':');
    hackerconvert.insert('x', '?');
    hackerconvert.insert('y', 'j');
    hackerconvert.insert('z', '2');
    

    println!("Ingresa texto:");
    io::stdin().read_line(&mut input).expect("No se puedo leer la consola");
    
    let mut lowercase = input.to_lowercase();
    let mut arrayconverter: Vec<char> = lowercase.chars().collect();
    
    let fullconverter: Vec<char> = arrayconverter
        .iter()
        .map(|&c| {
            hackerconvert
                .get(&c)
                .cloned() // Clona el valor o devuelve el carácter original
                .unwrap_or(c)
        })
        .collect();
    let stringconverter: String = String::from_iter(fullconverter);
    println!("Entrada hackeada:\n{}",stringconverter);


}
