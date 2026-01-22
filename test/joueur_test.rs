



#[test]
fn test_création_joueur_nommé(){
    let joueur = Joueur::new("Odin");
    assert_eq!(joueur.combien_de_bonne_réponse(), 0);
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 0);
    assert_eq!(joueur.voici_mon_nom(), "Odin");
}



#[test]
fn test_ajout_bonne_réponse(){
    let joueur = Joueur::new("Thor");
    joueur.ajout_bonne_réponse();
    assert_eq!(joueur.combien_de_bonne_réponse(), 1);
    for i in 0..10000{
        joueur.ajout_bonne_réponse();
    }
    assert_eq!(joueur.combien_de_bonne_réponse(), 10000);
}

#[test]
fn test_ajout_mauvaise_réponse(){
    let joueur = Joueur::new("Baldur");
    joueur.ajout_mauvaise_réponse();
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 1);

    for i in 0..10000{
        joueur.ajout_mauvaise_réponse();
    }
    assert_eq!(joueur.combien_de_mauvaise_réponse(), 10000);
}


#[test]
fn test_nom(){
    let joueur = Joueur::new("Týr");
    assert_eq!(joueur.voici_mon_nom(), "Týr");
}












