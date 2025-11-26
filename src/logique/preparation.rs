use std::io;
use crate::joueur::Joueur;
use crate::logique::preparer::préparer;


pub struct preparation;
impl préparer for preparation{
    fn crée_joueur(&self) -> Joueur {
        Joueur::nouveau()
    }

    fn demander_nb_manche(&self, taille_liste: usize) -> usize {
        loop {
            //crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
            let mut entree = String::new(); // Crée une nouvelle chaîne à chaque itération
            println!("Combien de manche ? ");
            let min = if taille_liste/2 < usize::MAX {  // les questions et les réponses sont déjà séparer, donc on divise par deux
                taille_liste/2
            } else {
                usize::MAX
            };
            println!("Nombre max de manches : {}", min.to_string());

            io::stdin()
                .read_line(&mut entree)
                .expect("Erreur lors de la lecture du nombre de manches");

            match entree.trim().parse::<usize>() {
                Ok(num) => {
                    if num <= min {
                        return num
                    }
                }, //  Retourne le nombre valide et quitte la boucle si le nombre n’est pas trop grand, sinon on va dépasser la taille de la liste
                Err(_) => println!("Entrée invalide, veuillez entrer un nombre entier positif."),
            }
        }
    }
}
