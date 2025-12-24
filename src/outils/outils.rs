use std::io;
use crate::joueur::Joueur;
use crate::outils::mot::{crée_liste, Question};
use std::error::Error;
use crossterm::{cursor, event::{self, Event, KeyCode}, execute, queue, terminal::{self, ClearType}};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use crossterm::cursor::MoveToColumn;
use crossterm::event::KeyEventKind;
use crossterm::style::{Color, SetForegroundColor};
use crossterm::terminal::Clear;
use crate::jeux::{Jeux, Mode};
use crate::outils::terminal::{afficher, afficher_str};




pub fn demander() -> String{
    let mut variable = String::new();
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_lowercase().to_string()
}


fn conv_vec_char_vers_string(chaine: &Vec<char>) -> String{
    chaine.into_iter().collect::<String>()
}


pub fn demander_réponse(liste_essai: &mut Vec<String>,nb_lettre: &usize,fin: Option<Instant>) -> Result<String,Box<dyn Error>>{
    // Active le mode "raw" pour lire les touches en direct
    terminal::enable_raw_mode()?;
    let mut sortie = stdout();
    let mut entrée: Vec<char> = Vec::new();
    let mut compteur = 1;

    sortie.flush()?;

    let mut position = 0;

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

                        entrée.insert(position,c);
                        execute!(sortie, MoveToColumn(0), Clear(ClearType::CurrentLine))?;
                        print!("{}", conv_vec_char_vers_string(&entrée));
                        position += 1;
                        execute!(sortie,MoveToColumn(position as u16))?;
                        sortie.flush()?;
                    }
                    KeyCode::Backspace => {
                        if !entrée.is_empty() && position > 0 {
                            entrée.remove(position - 1);
                            execute!(
                                sortie,
                                MoveToColumn(0),
                                Clear(ClearType::CurrentLine),

                            )?;
                            print!("{}", conv_vec_char_vers_string(&entrée));
                            position -= 1;
                            execute!(
                                sortie,
                                MoveToColumn(position as u16),
                            )?;
                        }
                    }
                    KeyCode::Delete =>{

                        if !entrée.is_empty() && position < entrée.len() {
                            entrée.remove(position);
                            execute!(
                                sortie,
                                MoveToColumn(0),
                                Clear(ClearType::CurrentLine),

                            )?;
                            print!("{}", conv_vec_char_vers_string(&entrée));
                            //position -= 1;
                            execute!(
                                sortie,
                                MoveToColumn(position as u16),
                            )?;
                        }
                    }

                    KeyCode::Enter => break,

                    KeyCode::Left => {
                        if position > 0{
                            execute!(sortie, cursor::MoveLeft(1))?;
                            position -= 1;
                        }

                    }

                    KeyCode::Right => {
                        if position < entrée.len() {
                            execute!(sortie, cursor::MoveRight(1))?;
                            position +=1;
                        }

                    }

                    KeyCode::Up => {
                        if liste_essai.len() >= compteur {
                            execute!(
                                sortie,
                                Clear(ClearType::CurrentLine),
                                MoveToColumn(0)
                            )?;

                            entrée = liste_essai[liste_essai.len() - compteur].clone().chars().collect();
                            compteur += 1;
                            position = entrée.len();
                            print!("{}", conv_vec_char_vers_string(&entrée));
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
                            entrée = liste_essai[liste_essai.len() - compteur].clone().chars().collect();
                            position = entrée.len();
                            print!("{}", conv_vec_char_vers_string(&entrée));
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
                let count = entrée.len();

                if *nb_lettre == count {
                    queue!(sortie, SetForegroundColor(Color::Green))?;
                } else if count >= 1 {
                    queue!(sortie, SetForegroundColor(Color::Red))?;
                }

                queue!(
                    sortie,
                    cursor::MoveTo(40, saved_cursor.1),
                    Clear(ClearType::UntilNewLine)
                )?;

                let caractère = if count <= 1 {"caractère"} else {"caractères"};

                write!(sortie, "{} {}", count,caractère)?;
                queue!(sortie, SetForegroundColor(Color::Reset))?;
                queue!(sortie, cursor::MoveTo(saved_cursor.0, saved_cursor.1))?;
                sortie.flush()?;




            }
        }

        if fin.is_some() {
            //affichage dynamique du temp
            let sauvegarde_position_curseur = cursor::position()?;
            let nombre = fin.unwrap() - Instant::now();


            if fin.unwrap() <= Instant::now() + Duration::from_secs(10) {
                queue!(sortie, SetForegroundColor(Color::Red))?;
            }

            queue!(
                    sortie,
                    cursor::MoveTo(40, sauvegarde_position_curseur.1-2),  //soustraction magique
                    Clear(ClearType::UntilNewLine)
                )?;

            let inscription = if nombre <= Duration::from_secs(1) { "seconde" } else { "secondes" };

            write!(sortie, "{} {}", nombre.as_secs(), inscription)?;
            queue!(sortie, SetForegroundColor(Color::Reset))?;
            queue!(sortie, cursor::MoveTo(sauvegarde_position_curseur.0, sauvegarde_position_curseur.1))?;
            sortie.flush()?;
        }

        if fin.is_some() {
            if fin.unwrap() <= Instant::now() {
                terminal::disable_raw_mode()?;
                return Ok("stop".to_string());
            }
        }

    }

    terminal::disable_raw_mode()?;
    //afficher(format!("\nEntrée finale : {}", entrée));
    Ok(entrée.into_iter().collect())
}


/// Demande si le joueur veut rejouer.
///
///
/// # Comportement
/// - Demande si le joueur veut rejouer
///
/// # Retour
/// - la réponse, sous forme de booléen. Oui pour rejouer, non sinon
pub fn rejouer() -> bool{
    afficher(String::from("\n\nrejouer ? "));

    loop{
        let réponse = demander();
        match réponse.as_str() {
            "oui" | "o" =>  return true,
            "non" | "n" => return false,
            _ => afficher_str("si tu n'arrive même pas a répondre a une question aussi simple tu n'es pas prêt pour la suite"),
        }
    }

}


pub fn crée_joueur() -> Joueur {
    Joueur::nouveau()
}

pub fn crée_partie(est_multi: bool, question: Option<Question>, mode: Option<Mode>, joueur: Option<Joueur>) -> Jeux {
    let question = if question.is_some() {question.unwrap()} else {crée_liste()};
    let mode = if mode.is_some() {mode.unwrap()} else {mode_de_jeu()};
    let joueur = if joueur.is_some() {joueur.unwrap()} else {crée_joueur()};

    Jeux::nouveau(mode, joueur, question, est_multi)
}


fn mode_de_jeu() -> Mode {
    afficher_str("Classique ? Chronomètre ? Survie ?");

    match demander().as_str() {
        "classique"  | "1" | "cl" => {
            Mode::nouveau("classique").unwrap()
        }
        "chronomètre" | "2" | "ch"  => {
            Mode::nouveau("chronomètre").unwrap()
        }
        "survie" | "3" | "s" | "su" => Mode::nouveau("survie").unwrap(),

        _ => {
            afficher_str("bon… bha on va dire classique alors…");
            Mode::nouveau("classique").unwrap()
        }
    }
}



pub fn demande_nom() -> String{
    afficher_str("Quel est ton nom ?");
    demander()
}


pub fn demander_temp() -> usize{
    loop {
        afficher_str("Combien de secondes ?");
        let entrée = demander();

        match entrée.parse::<usize>() {
            Ok(num) => {
                return num
            }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
            Err(_) => afficher_str("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }

}


pub fn demander_nb_manche(taille_liste: usize) -> usize {
    loop {

        afficher_str("Combien de manche ? ");
        let min = if taille_liste < usize::MAX {
            taille_liste
        } else {
            usize::MAX
        };
        afficher(format!("Nombre max de manches : {}", min.to_string()));
        let entree = demander();


        match entree.parse::<usize>() {
            Ok(num) => {
                if num <= min {
                    return num
                }
            }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
            Err(_) => afficher_str("Entrée invalide, veuillez entrer un nombre entier positif."),
        }
    }
}


pub fn transforme_vec_string_en_tuple_string(vecteur: Vec<String>) -> Vec<(String,String)> {
    let mut nouvelle_liste_2_0:Vec<(String,String)> = vec![];

    let mut compteur = 0;
    while compteur < vecteur.len() {
        nouvelle_liste_2_0.push((vecteur[compteur].to_string(), vecteur[compteur+1].to_string()));
        compteur += 2;
    }
    nouvelle_liste_2_0
}