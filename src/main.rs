use crate::multi_joueur::multi_joueur::multi_joueur;
use crate::solitaire::solitaire;

mod joueur;
mod mot;
mod affichage;
mod solitaire;
mod multi_joueur;
mod logique;

fn main() {
    println!("Mode de jeu ? solitaire ou multi_joueur ?");
    loop {
        let mut mode = String :: new();
        std::io::stdin()
            .read_line(&mut mode)
            .expect("Erreur lors de la saisie du mode");

        match mode.trim() {
            "solitaire" => {
                solitaire();
                println!("Lancement d'une nouvelle partie");
                println!("Mode de jeu ? solitaire ou multi_joueur ?");
            }
            "multi_joueur" => {
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
