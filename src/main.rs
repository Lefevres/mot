use std::fs;
use rand::RngCore;

fn main() {
    let mot = lire_fichier();
    println!("{}", mot);
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
