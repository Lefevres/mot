use std::io;
use crate::joueur::Joueur;
use crate::outils::mot::cree_liste;
use std::error::Error;
use crossterm::{cursor, event::{self, Event, KeyCode}, execute, queue, terminal::{self, ClearType}};
use std::io::{stdout, Write};
use crossterm::event::KeyEventKind;
use crossterm::style::{Color, SetForegroundColor};
use crossterm::terminal::Clear;

pub fn demander(mut variable:String) -> String{
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_string()
}


pub fn demander_réponse(liste_essai: &mut Vec<String>,nb_lettre: usize) -> Result<String,Box<dyn Error>>{
    // Active le mode "raw" pour lire les touches en direct
    terminal::enable_raw_mode()?;
    let mut sortie = stdout();
    let mut entrée = String::new();
    let mut compteur = 1;

    sortie.flush()?;

    loop {
        // attend un événement clavier
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                // Traiter uniquement la pression initiale (évite les doublons)
                if key.kind != KeyEventKind::Press {   //windows c’est vraiment nul !
                    continue;
                }

                match key.code {
                    KeyCode::Char(c) => {
                        entrée.push(c);
                        print!("{}", c);
                        sortie.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !entrée.is_empty() {
                            entrée.pop();
                            execute!(
                                sortie,
                                cursor::MoveLeft(1),
                                terminal::Clear(ClearType::UntilNewLine)
                            )?;
                        }
                    }
                    KeyCode::Enter => break,
                    KeyCode::Left => {
                        execute!(sortie, cursor::MoveLeft(1))?;
                    }
                    KeyCode::Right => {
                        execute!(sortie, cursor::MoveRight(1))?;
                    }
                    KeyCode::Up => {
                        if liste_essai.len() >= compteur {
                            execute!(
                                sortie,
                                Clear(ClearType::CurrentLine),
                                cursor::MoveToColumn(0)
                            )?;
                            entrée = liste_essai[liste_essai.len() - compteur].clone();
                            compteur += 1;
                            print!("{}", entrée);
                        }
                    }
                    KeyCode::Down => {
                        if compteur > 1 {
                            compteur -= 1;
                            execute!(
                                sortie,
                                Clear(ClearType::CurrentLine),
                                cursor::MoveToColumn(0)
                            )?;
                            entrée = liste_essai[liste_essai.len() - compteur].clone();
                            print!("{}", entrée);
                        } else {
                            execute!(
                                sortie,
                                Clear(ClearType::CurrentLine),
                                cursor::MoveToColumn(0)
                            )?;
                            entrée.clear();
                        }
                    }
                    _ => {}
                }

                // Affichage dynamique du compteur de lettres
                let saved_cursor = cursor::position()?;
                let count = entrée.chars().count();

                if nb_lettre == count {
                    queue!(sortie, SetForegroundColor(Color::Green))?;
                } else if count >= 1 {
                    queue!(sortie, SetForegroundColor(Color::Red))?;
                }

                queue!(
                    sortie,
                    cursor::MoveTo(40, saved_cursor.1),
                    Clear(ClearType::UntilNewLine)
                )?;
                write!(sortie, "{} caractères", count)?;
                queue!(sortie, SetForegroundColor(Color::Reset))?;
                queue!(sortie, cursor::MoveTo(saved_cursor.0, saved_cursor.1))?;
                sortie.flush()?;
            }
        }
    }

    terminal::disable_raw_mode()?;
    //println!("\nEntrée finale : {}", entrée);
    Ok(entrée)
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
        //crossterm::execute!(sortie(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();

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