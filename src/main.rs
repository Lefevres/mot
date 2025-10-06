use std::{fs, io};
use rand::RngCore;

fn main() {
    let mot = lire_fichier();
    println!("{}", mot);
    let separer = mot.split(":").collect::<Vec<&str>>();
    let mot = separer[0];
    let def = separer[1];

    //println!("{}", mot + def);
    println!("{}", def);
    essai(mot,def);



}

fn essai(mot: &str,def: &str) {
    let mut saisie = String::new();
    loop {
        println!("{}", def);
        saisie.clear();
        io::stdin()
            .read_line(&mut saisie)
            .expect("Erreur lors de la lecture");
        if saisie.trim().to_lowercase()   == mot.trim().to_lowercase() {
            println!("bravo tu as trouver !");
            break;
        }
        println!("{}", saisie.trim().to_lowercase());
        println!("{}",mot.trim());
    }
}

fn lire_fichier() -> String {
    let mut aleatoire = rand::rng();
    let mut int_aleatoire = aleatoire.next_u32();
    let fichier = fs::read_to_string("mot.txt").unwrap();
    let lignes: Vec<&str> = fichier.lines().collect();

    let nb_lignes = lignes.len();
    int_aleatoire = int_aleatoire % (nb_lignes-1) as u32;
    println!("{}", int_aleatoire);
    lignes[int_aleatoire as usize].to_string()


}
