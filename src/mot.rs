use std::fs;
use rand::prelude::SliceRandom;


const FICHIER: &str = "mot.txt";
pub fn cree_liste() -> Vec<String>{
    let fichier = lis_fichier();
    let liste = melange_liste(fichier);
    liste
}

fn lis_fichier() -> Vec<String>{
    let contenu = fs::read_to_string(FICHIER)
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