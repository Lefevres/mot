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
use crate::outils::terminal::afficher_str;



/// Fonction gérérale permettant de recevoir une entrée utilisateur et d’effectuer dessus quelque opération basique.
///
/// # Paramètre
/// - Un option de littérale de chaine de caractère
///
/// # Retour
/// - Renvoi un string de la saisie utilisateur
///
/// # Comportement
/// Affiche si fourni le paramètre,
/// puis demande une saisie utilisateur, la modifie et la renvoie.
///
/// # Opérations
/// - Supprime les espaces aux extremité de la saisie
/// - Remplace les lettres majuscules par des minuscules
///
pub fn demander(a_afficher :Option<&str>) -> String{
    if let Some(texte) = a_afficher
        && !texte.is_empty() {
            afficher_str(texte);
    }

    let mut variable = String::new();
    io::stdin()
        .read_line(&mut variable)
        .expect("il y a un problème dans demander de outils");
    variable.trim().to_lowercase()
}


/// Fonction gérérale permettant de transformer un vecteur de caractère en un string.
///
/// # Paramètre
/// - Un vecteur de caractères.
///
/// # Retour
/// - Renvoie une chaine de caractères.
///
/// # Comportement
/// - Transforme le vecteur de caractère passer en paramètre en chaine de caractère.
///
fn conv_vec_char_vers_string(chaine: &Vec<char>) -> String{
    chaine.iter().collect()
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
    loop{
        let réponse = demander(Some("rejouer"));
        match réponse.as_str() {
            "oui" | "o" =>  return true,
            "non" | "n" => return false,
            _ => afficher_str("si tu n'arrive même pas a répondre a une question aussi simple tu n'es pas prêt pour la suite"),
        }
    }

}


/// Fonction gérérale permettant de créer une partie de zéro ou non.
///
/// # Paramètre
/// - une potentielle Question
/// - un potentiel Mode
/// - un potentiel Joueur.
///
/// # Retour
/// - Une structure de Jeux.
///
/// # Comportement
/// Si les paramètres n’existent pas, ils sont définie,
/// enfin, on crée d’une structure Jeux avec ces paramètres.
///
pub fn crée_partie(question: Option<Question>, mode: Option<Mode>, joueur: Option<Joueur>) -> Jeux {
    let question = question.unwrap_or_else(crée_liste);
    let mode = mode.unwrap_or_else(crée_mode_de_jeu);
    let joueur = joueur.unwrap_or_else(Joueur::nouveau);

    Jeux::nouveau(mode, joueur, question)
}


/// Fonction générale permettant de créer un mode de jeu.
///
/// # Paramètre
/// - Aucun paramètre.
///
/// # Retour
/// - Une structure de Mode.
///
/// # Comportement
/// Demande le mode de jeu au joueur,
/// crée le Mode de jeux suivant le choix du joueur,
/// si le mode de jeux est mal choisi, on choisit par défaut le mode classique.
///
fn crée_mode_de_jeu() -> Mode {
    match demander(Some("Classique ? Chronomètre ? Survie ?")).as_str() {
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
    demander(Some("Quel est ton nom ?"))
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
        if event::poll(std::time::Duration::from_millis(50))?
            && let Event::Key(key) = event::read()? {
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

        if let Some(fin_val) = fin {
            //affichage dynamique du temp
            let sauvegarde_position_curseur = cursor::position()?;
            let nombre = fin_val - Instant::now();


            if fin_val <= Instant::now() + Duration::from_secs(10) {
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

        if let Some (fin_val) = fin
            && fin_val <= Instant::now() {
                terminal::disable_raw_mode()?;
                return Ok("stop".to_string());
            }

    terminal::disable_raw_mode()?;
    Ok(entrée.into_iter().collect())
}