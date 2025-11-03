use std::fs;
use std::path::PathBuf;
use rand::prelude::SliceRandom;

const FICHIER: &str = "truc_utile/mot.txt";
pub fn cree_liste() -> Vec<String>{
    let fichier = lis_fichier();
    let liste = melange_liste(fichier);
    liste
}

fn lis_fichier() -> Vec<String>{
    let home = std::env::var("HOME").expect("Pas de variable HOME !");
    let mut path = PathBuf::from(home);
    path.push(FICHIER);
    let contenu = fs::read_to_string(&path)
        .expect("Erreur lecture fichier")
        .lines()  // découpe en lignes
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contenu
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