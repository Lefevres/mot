use std::{fs, path::PathBuf};
use std::env::home_dir;
use std::string::ToString;
use std::sync::LazyLock;
use rand::prelude::SliceRandom;
use crate::outils::outils::transforme_vec_string_en_tuple_string;

static CHEMIN: LazyLock<PathBuf> = LazyLock::new(|| {
    home_dir().expect("Impossible de trouver le dossier home").join(".mot")
});
static FICHIER: LazyLock<PathBuf> = LazyLock::new(|| CHEMIN.join("mot.txt")); //j'ai retirer //../../


pub fn cree_liste() -> Vec<(String, String)> {
    let fichier = lis_fichier();
    let liste = melange_liste(fichier);
    liste
}


fn lis_fichier() -> Vec<String>{
    let contenu = fs::read_to_string(FICHIER.clone())
        .expect("Erreur lecture fichier mot")
        .lines()  // découpe en lignes
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    contenu
}

fn melange_liste(mut liste:Vec<String>) -> Vec<(String,String)>{
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

    transforme_vec_string_en_tuple_string(nouvelle_liste)
}