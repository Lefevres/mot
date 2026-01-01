use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use crate::jeux::Jeux;
use crate::outils::outils::{demande_nom, demander, rejouer};
use crate::outils::terminal::{afficher, afficher_str};


const PORT: &str = ":9000";


pub fn client(){

    let nom = demande_nom();

    let mut stream = connection();


    envoie_a_l_hote(&mut stream, nom.clone()).expect("J'envoie le nom");


    afficher_str("On attend que tout le monde soit prêt…");

    loop{
        
        let mut jeux = récupéré_jeux(&mut stream).unwrap();



        // Lance la partie

        let résultat = jeux.jouer();

        //let résultat = jouer(&mut joueur, &liste, nb_manche);
        let résultat = résultat.0.to_string() +";"+ &résultat.1.to_string();

        envoie_a_l_hote(&mut stream, résultat).expect("on a un soucis");

        let résultats = reçoit_les_résultats(&mut stream,&nom);

        afficher_résultat(résultats);

        if !rejouer() {
            envoie_a_l_hote(&mut stream, "n".to_string()).expect("on a un soucis");
            break;
        }
        else {
            envoie_a_l_hote(&mut stream, "o".to_string()).expect("bon bha ça marche pas");
            afficher_str("on attend tout les joueurs");
        }
    }

}


 fn récupéré_jeux(socket: &mut TcpStream) -> Option<Jeux> {



    let mut reader = BufReader::new(socket);

    let mut jeux = String::new();

    reader.read_line(&mut jeux).expect("je panic");

    if jeux.is_empty() {
        afficher_str("J’ai pas trouver, le serveur a fermer la connexion ?");
        return None
    }



    let jeux = serde_json::from_str::<Jeux>(&jeux);
    match jeux {
        Ok(jeux) => { Some(jeux) },
        Err(e) => {eprintln!("Erreur de désérialisation : {}", e);None},
    }


}


fn afficher_résultat(résultats:Vec<(String,usize,usize)>)  {
    afficher_str("\n");
    for résultat in résultats {
        let nom = résultat.0;
        let bonne_réponse = résultat.1;
        let mauvaise_réponse = résultat.2;
        let total = bonne_réponse + mauvaise_réponse;
        let ratio = if total > 0 {
            (bonne_réponse as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        afficher(format!("{} a eu {} bonne réponse(s) et {} mauvaise(s) pour un ration de {:.1}%\n",nom,bonne_réponse,mauvaise_réponse,ratio));
    }
}


fn reçoit_les_résultats(socket: &mut TcpStream,mon_nom : &String) -> Vec<(String,usize,usize)> {
    let message = lis_message(socket).unwrap();
    let message = message.trim().to_string();
    let préparation_retour = message.split(";")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut résultats:Vec<(String, usize, usize)> = Vec::new();
    for i in (0..préparation_retour.len()).step_by(3) {
        let nom = &préparation_retour[i];
        if nom == mon_nom{
            continue;
        }

        match préparation_retour.get(i+1) {
            Some(_) => (),
            None => eprintln!("Erreur de conversion"),
        }
        match préparation_retour.get(i+2) {
            Some(_) => (),
            None => eprintln!("Erreur de conversion"),
        }

        let bonne_réponse = préparation_retour[i+1].parse().unwrap();
        let mauvaise_réponse = préparation_retour[i+2].parse().unwrap();
        résultats.push((nom.to_string(),bonne_réponse,mauvaise_réponse));
    }
    résultats
}


fn envoie_a_l_hote(stream : &mut TcpStream, message:String) -> std::io::Result<()>{
    let message_bytes = message.as_bytes();
    stream.write_all(message_bytes)?;
    Ok(())
}


fn lis_message(stream : &mut TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer)?;
    if n == 0 {
        return Err("Le serveur a fermé la connexion".into());
    }
    let message = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(message)
}

/// Permet au client de rejoindre une partie.
///
///
/// # Comportement
/// - Se connecte à l'hote via TCP
/// - Renvoie le socket
///
fn connection() -> TcpStream {
    let ip = demander(Some("Quelle adresse ip ? (\"ip a\" sous linux, ip config sous windows)"));

    // Adresse IP du serveur
    let addresse = ip + PORT;

    afficher(format!("Connexion au serveur {}...", &addresse));
    let socket;
    loop{
        match TcpStream::connect(&addresse) {
            Ok(stream) => {
                afficher_str("Connecté !");
                socket = stream;
                break
            }
            Err(_e) => {
                std::thread::sleep(std::time::Duration::from_secs(3));

            }
        }
    }

    socket
}