use mot::question::definition::Definition;
use mot::question::vocabulaire::Vocabulaire;
use std::fs;

#[test]
fn test_création(){

    let fichier_de_test = "tests/fichier/test_definition.txt";

    fs::write(
        fichier_de_test,
        "esse :Crochet en forme de S.
         ire :Colère.",
    ).unwrap();


    let mut definition = Definition::nouveau(fichier_de_test);
    assert_eq!(definition.nombre_définition(), 2);
    assert_eq!(definition.suivant().unwrap(), "Crochet en forme de S.");
    assert_eq!(definition.suivant().unwrap(), "Colère.");
    
    assert!(definition.suivant().is_none());

    fs::remove_file(fichier_de_test).unwrap();
} 
