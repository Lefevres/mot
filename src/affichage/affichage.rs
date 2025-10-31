pub trait Affichage {
    fn afficher_en_tete();
    fn afficher_question(nb_question:usize, liste:Vec<&String>);
    fn afficher_indice(mot:&String);
}