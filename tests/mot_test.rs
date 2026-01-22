use mot::question::mot::Mot;
use mot::question::vocabulaire::Vocabulaire;
use std::fs;

#[test]
fn test_création(){

    let fichier_de_test = "tests/fichier/test_mot.txt";

    fs::write(
        fichier_de_test,
        "esse :Crochet en forme de S.
        ire :Colère.",
    ).unwrap();


    let mut mot = Mot::nouveau(fichier_de_test);
    assert_eq!(mot.nombre_mot(), 2);
    assert_eq!(mot.suivant().unwrap(), "esse");
    assert_eq!(mot.suivant().unwrap(), "ire");
    
    assert!(mot.suivant().is_none());

    fs::remove_file(fichier_de_test).unwrap();
} 
