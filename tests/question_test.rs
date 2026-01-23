use crate::question::Question;
use mot::question::question;
use std::fs;


#[test]
fn test_création(){
    let fichier_de_test = "tests/fichier/test_question.txt";
    fs::write(
        fichier_de_test,
        "
        esse : Crochet en forme de S.
        ire : Colère.
        ",
        ).unwrap();

    let question = Question::nouveau_avec_fichier(fichier_de_test).unwrap();

    assert_eq!(question.numéro_actuel(), 0);


    fs::remove_file(fichier_de_test).unwrap();
}


#[test]
fn test_et_après(){
    
    let fichier_de_test = "tests/fichier/test_question2.txt";
    fs::write(
        fichier_de_test,
        "
        esse : Crochet en forme de S.
        ire : Colère.
        ",
        ).unwrap();

    let mut question = Question::nouveau_avec_fichier(fichier_de_test).unwrap();
    
    assert_eq!(question.et_après().unwrap(), ("Crochet en forme de S.".to_string(), "esse".to_string()));
    assert_eq!(question.et_après().unwrap(), ("Colère.".to_string(), "ire".to_string()));

    assert!(question.et_après().is_err());

    
    fs::remove_file(fichier_de_test).unwrap();
}


#[test]
fn test_numéro_actuel(){
    
    let fichier_de_test = "tests/fichier/test_question3.txt";
    fs::write(
        fichier_de_test,
        "
        esse : Crochet en forme de S.
        ire : Colère.
        ",
        ).unwrap();

    let mut question = Question::nouveau_avec_fichier(fichier_de_test).unwrap();

    assert_eq!(question.numéro_actuel(), 0);
    let _ = question.et_après();    
    assert_eq!(question.numéro_actuel(), 1);
    let _ = question.et_après();
    assert_eq!(question.numéro_actuel(), 2);

    fs::remove_file(fichier_de_test).unwrap();
}



#[test]
fn test_combien(){
    
    let fichier_de_test = "tests/fichier/test_question4.txt";
    fs::write(
        fichier_de_test,
        "
        esse : Crochet en forme de S.
        ire : Colère.
        ",
    ).unwrap();

    let mut question = Question::nouveau_avec_fichier(fichier_de_test).unwrap();


    assert_eq!(question.combien(), 0);
    let _ = question.et_après();

    assert_eq!(question.combien(), 1);    

    fs::remove_file(fichier_de_test).unwrap();
}



#[test]
fn test_combien_en_tout(){
    
    let fichier_de_test = "tests/fichier/test_question5.txt";
    fs::write(
        fichier_de_test,
        "
        esse : Crochet en forme de S.
        ire : Colère.
        ",
    ).unwrap();

    let question = Question::nouveau_avec_fichier(fichier_de_test).unwrap();


    assert_eq!(question.combien_en_tout(), 2);


    fs::remove_file(fichier_de_test).unwrap();
} 


