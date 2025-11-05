use std::{env, fs};
use std::path::{Path, PathBuf};
use rand::prelude::SliceRandom;

const FICHIER: &str = "mot.txt"; //".local/bin/mot.txt";
pub fn cree_liste() -> Vec<String>{
    let fichier = lis_fichier();
    let liste = melange_liste(fichier);
    liste
}

fn lis_fichier() -> Vec<String>{

    let home = std::env::var("HOME").expect("Pas de variable HOME !");
    let mut chemin = PathBuf::from(home);
    chemin.push(FICHIER);
    //mise_en_place(&chemin);
    let contenu = fs::read_to_string(&chemin)
        .expect("Erreur lecture fichier")
        .lines()  // découpe en lignes
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contenu
}

fn mise_en_place(chemin: &Path){
    if !Path::new(chemin).exists() {
        let source = "mot.txt";

        let home = env::var("HOME").expect("Pas de variable HOME !");
        let mut destination = PathBuf::from(home);
        destination.push(".local/bin/mot.txt");


        match fs::copy(source, destination) {
            Ok(bytes) => println!("Fichier copié avec succès ({} octets)", bytes),
            Err(e) => eprintln!("Erreur lors de la copie : {}", e),
        }
    }


}

fn melange_liste(mut liste:Vec<String>) -> Vec<String>{
    let mut rng = rand::rng();
    liste.shuffle(&mut rng);

    // On découpe les lignes en mots et on convertit tout en String
    let nouvelle_liste = liste
        .into_iter()
        .flat_map(|ligne| {
            ligne
                .split(':')
                .map(|s| s.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<String>>();

    nouvelle_liste
}