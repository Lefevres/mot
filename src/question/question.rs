

pub struct Question{
    curseur: usize,
    nombre_question: usize,
    liste_définition: Definition,
    liste_mot: Mot,
}

impl Question{
    Question(){
        let fichier1 = fs::read_to_string(home_dir().join(".mot").join("mot.txt"))?;
        let mut un;
        for ligne in fichier
    }
}
