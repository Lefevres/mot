use std::io;
use crate::joueur::Joueur;
use crate::outils::mot::cree_liste;
use std::error::Error;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
};
use std::io::{stdout, Write};



pub fn demander(mut variable:String) -> String{
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_string()
}


pub fn demander_réponse() -> Result<(String),Box<dyn Error>>{
    // Active le mode "raw" pour lire les touches en direct
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    let mut input = String::new();


    stdout.flush()?;

    loop {
        // attend un événement clavier
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                    }
                    KeyCode::Backspace => {
                        if !input.is_empty() {
                            input.pop();
                            // effacer un caractère
                            execute!(
                                stdout,
                                cursor::MoveLeft(1),
                                terminal::Clear(ClearType::UntilNewLine)
                            )?;
                        }
                    }
                    KeyCode::Enter => {
                        break;
                    }
                    _ => {}
                }

                // Affichage dynamique du compteur
                let saved_cursor = cursor::position()?;

                let count = input.chars().count();

                // aller à droite (colonne 40 par exemple)
                execute!(
                    stdout,
                    cursor::MoveTo(40, saved_cursor.1),
                    terminal::Clear(ClearType::UntilNewLine)
                )?;
                print!("{} caractères", count);

                // Retour où était le curseur
                execute!(stdout, cursor::MoveTo(saved_cursor.0, saved_cursor.1))?;
                stdout.flush()?;
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!("\nEntrée finale : {}", input);
    Ok(input)
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