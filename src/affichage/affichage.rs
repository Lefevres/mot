pub trait Affichage {
    fn afficher_en_tete(&self);
    fn afficher_question<'a>(&self, nb_question: usize, liste: &'a Vec<String>) -> &'a String;
    fn afficher_indice(&self, mot:&String);

    fn afficher_reponse_precedante(&self, mot:&String);

    fn afficher_bonne_reponse(&self);

    fn afficher_mauvaise_reponse(&self);
}