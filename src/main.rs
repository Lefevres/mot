use std::fs;

fn main() {

    lire_fichier();
    println!("Hello, world!");
}

fn lire_fichier() {
    let fichier = fs::read_to_string("mot.txt");

    for mot in fichier.unwrap().lines() {
        println!("{:?} \n", mot);
    }
}
