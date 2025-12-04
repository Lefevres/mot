use std::io;
use crate::joueur::Joueur;
use crate::outils::mot::cree_liste;


pub fn demander(mut variable:String) -> String{
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_string()
}


pub fn crée_joueur(est_multi:bool) -> Joueur {
    Joueur::nouveau(est_multi)
}


pub fn se_préparer<'a>(role : String) -> (Joueur,Vec<String>,usize,String){  //rajouter la demande de nom ?

    let joueur;
    let mut liste=Vec::new();
    let mut nb_manche= 0;
    let mut nom = String::new();

    match role.as_str() {
        "solitaire" => {
            joueur = crée_joueur(false);
            liste = cree_liste();
            nb_manche = demander_nb_manche(liste.len());
        }
        "client" => {
            joueur = crée_joueur(false);
            nom = demande_nom();
        }
        "hote" => {
            joueur = crée_joueur(false);
            liste = cree_liste();
            nom = demande_nom();
            nb_manche = demander_nb_manche(liste.len());

        }
        _ =>{
            joueur = crée_joueur(false);
            liste = cree_liste();
            eprintln!("attention je suis dans se_préparer et je suis un role qui n'existe pas");
        }
    }


    (joueur,liste,nb_manche,nom)
}


fn demande_nom() -> String{
    println!("Quel est ton nom ?");
    demander(String::new())
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