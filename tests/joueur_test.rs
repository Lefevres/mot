use mot::joueur::Joueur; 



#[test]
fn test_création_joueur_nommé(){
    let joueur = Joueur::nouveau("Odin");
    assert_eq!(joueur.combien_de_bonne_réponse(), 0);
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 0);
    assert_eq!(joueur.voici_mon_nom(), "Odin");
}



#[test]
fn test_ajout_bonne_réponse(){
    let mut joueur = Joueur::nouveau("Thor");
    joueur.ajout_bonne_réponse();
    assert_eq!(joueur.combien_de_bonne_réponse(), 1);
    for _ in 0..9{
        joueur.ajout_bonne_réponse();
    }
    assert_eq!(joueur.combien_de_bonne_réponse(), 10);
}

#[test]
fn test_ajout_mauvaise_réponse(){
    let mut joueur = Joueur::nouveau("Baldur");
    joueur.ajout_mauvaise_réponse();
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 1);

    for _ in 0..9{
        joueur.ajout_mauvaise_réponse();
    }
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 10);
}



















