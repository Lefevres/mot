use crate::multi_joueur::multi_joueur::multi_joueur;
use crate::solitaire::solitaire;

mod joueur;
mod mot;
mod affichage;
mod solitaire;
mod multi_joueur;
pub mod jouer;
pub mod preparation;

fn main() {
    println!("Mode de jeu ? solitaire(1) ou multi_joueur(2) ?");
    loop {
        let mut mode = String :: new();
        std::io::stdin()
            .read_line(&mut mode)
            .expect("Erreur lors de la saisie du mode");

        match mode.trim() {
            "solitaire" | "1" => {
                solitaire();
                println!("Lancement d'une nouvelle partie");
                println!("Mode de jeu ? solitaire ou multi_joueur ?");
            }
            "multi_joueur" | "2" => {
                multi_joueur();
                println!("Lancement d'une nouvelle partie");
                println!("Mode de jeu ? solitaire ou multi_joueur ?");
            }
            _ => {
                println!("N'importe quoi !!");
            }
        }

    }
}
