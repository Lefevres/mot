use std::io::stdout;
use colored::Colorize;
use crate::joueur::Joueur;
use crate::outils::outils::demander;




    pub fn afficher_en_tete(){
        crossterm::execute!(stdout(), crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).unwrap();
        let en_tete = "\n\nLes mots sont sans majuscule mais avec accent, les locutions latine ne prenne pas de tiret".green();
        let instruction = "\n +1 points pour une bonne réponse, -1 points pour une mauvaise réponse\n".red();
        let indices = "{indice} pour avoir le nombre de lettres\n{passe} pour changer de mot\n{stop} pour arrêter".red();
        println!("{en_tete}\n{instruction}\n{indices}");
    }

    pub fn afficher_question<'a>(nb_question : usize, liste : &'a Vec<String>) -> &'a String{  //renvoie le mot attendu
            let question = format!("{}", liste[nb_question+1]).bright_yellow();
            println!("\n\n{}\n\n", question ); //nb_question est la réponse, nb_question +1 est la question
            &liste[nb_question]
        }


    pub fn afficher_indice(mot: &String) {
        let len = mot.chars().count();
        let revelation = len / 3;

        let prefix: String = mot.chars().take(revelation).collect();

        let mut indice = prefix;
        for _ in 0..(len - revelation) {
            indice.push_str("_ ");
        }

        println!("Le mot a {} lettres", len);
        println!("{}", indice);
    }

    pub fn afficher_reponse_precedante(mot : &String){
            println!("La réponse étais {}\n",mot.green());
            println!("Entrer pour continuer : \n");
            demander(String::new());
        }

    pub fn afficher_bonne_reponse() {
            println!("Félicitation ce fût une bonne réponse 👍\n");
            println!("Entrer pour continuer : \n");
            demander(String::new());
        }

    pub fn afficher_mauvaise_reponse() {
            println!("Ça n'est malheureusement pas ça \n");
        }

    pub fn afficher_score(joueur: &mut Joueur, nb_manche: usize) {

            let total = joueur.bonne_reponse() + joueur.mauvaise_reponse();
            let ratio = if total > 0 {
                (joueur.bonne_reponse() as f32 / total as f32) * 100.0
            } else {
                0.0
            };
            println!(
                "\nScore : {} bonne(s) réponse(s), {} mauvaise(s) réponse(s) — Ratio : {:.1}%            Question {}/{}",
                joueur.bonne_reponse(), joueur.mauvaise_reponse(), ratio, joueur.question()/2+1,nb_manche
            );

        }

    pub fn afficher_score_fin(joueur: &mut Joueur) {

        let total = joueur.bonne_reponse() + joueur.mauvaise_reponse();
        let ratio = if total > 0 {
            (joueur.bonne_reponse() as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        println!(
            "\nScore : {} bonne(s) réponse(s), {} mauvaise(s) réponse(s) — Ratio : {:.1}%",
            joueur.bonne_reponse(), joueur.mauvaise_reponse(), ratio
        );

}

    pub fn afficher(texte: String) {
            println!("{}", texte);
        }

