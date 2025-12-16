use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc, Color};
use druid::widget::{Flex, Label};
use crate::affichage::affichage::Affichage;
use crate::joueur::Joueur;

#[derive(Clone, Data, Lens)]
pub struct AffichageGraphique {
    pub(crate) score: i32,
}

impl Affichage for AffichageGraphique {
    fn afficher_en_tete(&self) {
        todo!()
    }

    fn afficher_question<'a>(&self, nb_question: usize, liste: &'a Vec<(String, String)>) {
        todo!()
    }

    fn afficher_indice(&self, mot: &String) {
        todo!()
    }

    fn afficher_reponse_precedante(&self, mot: &String) {
        todo!()
    }

    fn afficher_bonne_reponse(&self) {
        todo!()
    }

    fn afficher_mauvaise_reponse(&self) {
        todo!()
    }

    fn afficher_score(&self, joueur: &Joueur, nb_manche: usize) {
        todo!()
    }

    fn afficher_str(&self, texte: &str) {
        println!("{}", texte);
    }

    fn afficher_score_fin(&self, joueur: Joueur) {
        todo!()
    }

    fn attendre_validation(&self) {
        todo!()
    }

    fn afficher(&self, texte: String) {
        // Créez une fenêtre avec Druid
        let main_window = WindowDesc::new(
            Flex::column()
                .with_child(
                    // Crée un label affichant le score
                    Label::new(format!("Score : {}", self.score))
                        .center()
                        .background(Color::WHITE)
                        .padding(10.0),
                )
                .boxed(),
        )
            .title("Jeu avec Affichage Graphique"); // Titre de la fenêtre

        // L'état initial est l'instance de `AffichageGraphique` avec un score
        let initial_state = AffichageGraphique { score: 42 };

        // Démarre l'application avec la fenêtre
        AppLauncher::with_window(main_window)
            .launch(initial_state)
            .expect("Échec de l'initialisation de l'interface graphique");
    }
}
