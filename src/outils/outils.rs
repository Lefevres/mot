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
use crate::jeux::Jeux;
use crate::mode::chronometre::Chronomètre;
use crate::mode::classique::Classique;
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




pub fn crée_joueur() -> Joueur {
    Joueur::nouveau()
}


/*pub fn crée_partie(est_multi: bool, question: Option<Question>, mode: Option<String>, joueur: Option<Joueur>){
    let question = if question.is_some() {question.unwrap()} else {crée_liste()};
    let mode = if mode.is_some() {mode.unwrap()} else {mode_de_jeu()};
    let joueur = if joueur.is_some() {joueur.unwrap()} else {crée_joueur()};

}*/



pub fn crée_partie(est_multi: bool, question: Option<Question>, mode: Option<String>, joueur: Option<Joueur>) -> Box<dyn Jeux> {
    let question = if question.is_some() {question.unwrap()} else {crée_liste()};
    let mode = if mode.is_some() {mode.unwrap()} else {mode_de_jeu()};
    let joueur = if joueur.is_some() {joueur.unwrap()} else {crée_joueur()};
    let jeux: Box<dyn Jeux>;

    match mode.as_str() {
        "classique" => jeux = Box::new(Classique::nouveau(joueur, question)),
        "chronomètre" => jeux = Box::new(Chronomètre::nouveau(joueur, question)),
        //"survie" => jeux = Mode::ModeSurvie(Survie::new(joueur, question)),
        _ => {
            println!("Mais qu'est ce que je fais là ? ");
            jeux = Box::new(Classique::nouveau(joueur, question));
        }
    }
    jeux

}


fn mode_de_jeu() -> String {
    afficher_str("Classique ? Chronomètre ? Survie ?");

    match demander().as_str() {
        "classique"  | "1" | "cl" => {
            "classique".to_string()
        }
        "chronomètre" | "2" | "ch"  => {
            "chronomètre".to_string()
        }
        "survie" | "3" | "s" | "su" => "survie".to_string(),

        _ => {
            afficher_str("bon… bha on va dire classique alors…");
            "classique".to_string()
        }
    }
}



pub fn demande_nom() -> String{
    afficher_str("Quel est ton nom ?");
    demander()
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