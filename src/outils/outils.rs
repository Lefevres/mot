use std::io;
use crate::joueur::Joueur;
use crate::mot::cree_liste;

pub fn demander(mut variable:String) -> String{
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_string()
}

pub fn crée_joueur(est_multi:bool) -> Joueur {
    Joueur::nouveau(est_multi)
}

pub fn se_préparer<'a>(multi : bool) -> (Joueur,Vec<String>,usize){
    let mut joueur = crée_joueur(multi);
    let liste = cree_liste();
    let mut nb_manche= 0;
    if !multi {
        nb_manche = demander_nb_manche(liste.len());
    }

    (joueur,liste,nb_manche)
}

pub fn demander_nb_manche(taille_liste: usize) -> usize {
    loop {
        //crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

        println!("Combien de manche ? ");
        let min = if taille_liste/2 < usize::MAX {  // les questions et les réponses sont déjà séparer, donc on divise par deux
            taille_liste/2
        } else {
            usize::MAX
        };
        println!("Nombre max de manches : {}", min.to_string());
        let entree = demander(String::new());


        match entree.parse::<usize>() {
            Ok(num) => {
                if num <= min {
                    return num
                }
            }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
            Err(_) => println!("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }
}