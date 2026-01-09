use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::jeux::{Jeux, Mode};
use crate::joueur::Joueur;
use crate::outils::mot::{crée_liste, Question};
use crate::outils::outils::{crée_partie, demande_nom, demander, rejouer};
use crate::outils::terminal::{afficher, afficher_str};



pub fn hote(){


    let mut nb_client:usize= demander_nb_joueur();


    let mut jeux = crée_partie(None, None, None);

    jeux.joueur.défini_nom(demande_nom());

    let clients = connextion_au_client(nb_client).unwrap();



    //pour rejouer
    let mut noms = clients.0;

    let mut sockets = clients.1;

    noms.insert(0, jeux.joueur.nom());

    loop{

        //l'hote envoi les jeux aux clients
        envoi_jeux(&mut sockets, jeux.mode().clone(), jeux.question().clone());


        let mut résultats:Vec<(String, String)> = Vec::new();





        //let mut jeux = Jeux::nouveau(jeux.mode().clone(), jeux.joueur.clone(), jeux.question().clone(), true);

        jeux.joueur.remet_les_questions_a_zero();
        jeux.jouer();


        if nb_client > 0{
            résultats = met_a_jour_les_résultats(&mut sockets);
        }


        résultats = ajoute_mes_résultats(résultats, &jeux.joueur);



        if !résultats.is_empty() {
            afficher_résultat(nb_client,&noms,jeux.joueur.nom(),&résultats);
        }



        if nb_client > 0 {
            partage_résultat(&mut sockets,&résultats,&noms);
        }
        else {
            afficher_str("tu es tout seul");
        }


        if !rejouer(){
            break;
        }
        else {
            afficher_str("on attend les autres joueurs");
            let a_retirer = joueur_restant(&mut sockets);

            if nb_client != a_retirer.len() {
                for nombre in a_retirer {
                    noms.remove(nombre+1); //il y a le nom de l'hote
                    sockets.remove(nombre);
                }
            }
            else {
                let nom = noms.first().unwrap();
                noms = Vec::from([nom.clone()]);
                sockets = Vec::new();
            }

        }
        nb_client = sockets.len();
        jeux.change_question(crée_liste());


    }
}

fn joueur_restant(sockets :&mut Vec<TcpStream>) -> Vec<usize>{
    let mut a_retirer:Vec<usize> = Vec::new();
    for (compteur, socket) in sockets.iter_mut().enumerate() {
        let message = lis_buffer(socket).unwrap();
        if message == "n" {
            a_retirer.push(compteur);
        }
    }
    a_retirer
}


fn envoi_jeux(sockets: &mut Vec<TcpStream>, mode: Mode, question: Question){
    for socket in sockets {
        let jeux = Jeux::nouveau(mode.clone(), Joueur::nouveau(), question.clone());
        let jeux_string = serde_json::to_string(&jeux).unwrap();
        envoie_message(socket,jeux_string);
    }
}



fn met_a_jour_les_résultats(sockets :&mut Vec<TcpStream>) -> Vec<(String,String)> {
    let mut résultats: Vec<(String, String)> = Vec::new();
    for socket in sockets {
        let buffer = lis_buffer(socket).unwrap();
        let (bonne_réponse, mauvaise_réponse) = buffer.split_once(";").unwrap();
        let résultat = (bonne_réponse.to_string(), mauvaise_réponse.to_string());
        résultats.push(résultat);
    }


    résultats
}


fn afficher_résultat(nb_client:usize, noms :&Vec<String>, mon_nom :String, résultats :&Vec<(String,String)>) {
    afficher_str("\n");

    for i in 0..nb_client+1 { //pour l'hote
        let nom = noms[i].clone();
        if *nom == mon_nom{
            continue;
        }

        let bonne_réponse:usize = résultats[i].0.parse().unwrap();
        let mauvaise_réponse:usize = résultats[i].1.parse().unwrap();
        let total = bonne_réponse+mauvaise_réponse;
        let ratio = if total > 0 {
            (bonne_réponse as f32 / total as f32) * 100.0
        } else {
            0.0
        };
        afficher(format!("{} a eu {} bonne réponse(s) pour {} mauvaise(s) pour un ration de {:.1}% \n",nom, résultats[i].0, résultats[i].1,ratio));
    }
}


fn ajoute_mes_résultats(mut résultats: Vec<(String, String)>, moi:&Joueur) -> Vec<(String, String)> {
    let mes_résultats = (moi.bonne_reponse().to_string(),moi.mauvaise_reponse().to_string());
    résultats.insert(0,mes_résultats);
    résultats
}


fn partage_résultat(sockets: &mut Vec<TcpStream>,résultats: &Vec<(String,String)>, noms :&Vec<String>){
    let mut message = "".to_string();
    for i in 0..noms.len() {
        message += &*noms[i]; //nom;br;mr
        message += ";";
        message += &résultats[i].0;
        message += ";";
        message += &résultats[i].1;
        message += ";";
    }
    message.pop();
    for socket in sockets {
        envoie_message(socket,message.clone());
    }
}


fn demander_nb_joueur() -> usize {
    loop {
        let nb_joueur = demander(Some("Pour combien de joueur ? (hormis toi)"));

        if nb_joueur.parse::<i32>().is_ok(){
            return nb_joueur.parse::<i32>().unwrap() as usize;
        }
    }
}


fn lis_buffer(socket:&mut TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0u8; 1024];
    let n = socket.read(&mut buffer)?;
    if n == 0 {
        return Err("Le client a fermé la connexion".into());
    }
    let littérale = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(littérale)

}


fn envoie_message(socket:&mut TcpStream, message: String){
    let message = message + "\n";
    let message_bytes = message.as_bytes();
    socket.write_all(message_bytes).unwrap();
}


fn connextion_au_client(nb_client: usize) -> Result<(Vec<String>,Vec<TcpStream>), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    let mut noms_joueurs = Vec::new();
    let mut sockets = Vec::new();
    for _ in 0..nb_client {
        if nb_client > 1 {
            afficher_str("En attente des joueurs...");
        }else{
            afficher_str("En attente du joueur...");
        }

        let (mut socket, adresse) = listener.accept()?;
        afficher(format!("Client connecté : {}", adresse));

        let nom = lis_buffer(&mut socket)?;

        noms_joueurs.push(nom);
        sockets.push(socket);
    }

    Ok((noms_joueurs,sockets))
}


