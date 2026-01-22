
#[test]
#[should_panic]
fn test_fichier_inexistant(){
    let mut question = Question::nouveau("ce fichier n'existe pas !!");
    question = question.unwrap();
}

#[test]
fn test_lire_fichier(){
    let mut fichier = match File::create("fichier_test_1") {
        Err(e) => panic!("Je n'est pas réussi a crée le fichier : fichier_test_1"),
        Ok() => _,
    };

    match fichier.write_all("esse : Crochet en forme de S.".as_bytes()){
        Err(e) => panic!("Je n'est pas pu écrire dans : fichier_test_1");
        Ok() => _,
    };

    let mut question = Question::nouveau("fichier_test_1").unwrap();
    let voc = question.question().unwrap();
    assert_eq!(voc.0, "Crochet en forme de S.");
    assert_eq!(voc.1, "esse");

}
