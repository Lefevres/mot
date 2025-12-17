use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener,TcpStream};
use crate::jeux::{Jeux, Mode};
use crate::joueur::Joueur;
use crate::outils::outils::{demander, demander_nb_manche, demander_temp, se_préparer};
use crate::outils::terminal::{afficher, afficher_str};

#[tokio::main]
pub async fn hote(){

    let mode = mode_de_jeu();
    let nb_client:usize= demander_nb_joueur();

    let (joueur, liste, mon_nom, nb_manche) = se_préparer("hote");

    let clients = connextion_au_client(nb_client).await.unwrap();

    let mut noms = clients.0;

    let mut sockets = clients.1;


    //l'hote envoi les jeux aux clients
    envoi_jeux(&mut sockets, mode.clone(), liste.clone()).await;

    let mut option = false;
    let mut info = 0;

    match mode {

        Mode::Classique => {
            option = true;
            info = demander_nb_manche(liste.len());
            envoi_message_tous(&mut sockets,&info.to_string()).await;
        }
        Mode::Chronomètre => {
            option = true;
            info = demander_temp();
            envoi_message_tous(&mut sockets,&info.to_string()).await;
        }
        _ => ()
    }
   // message_initialisation(&mut sockets, nb_manche, &liste[0..nb_manche].to_vec(),mode.clone()).await;  //fois deux pour question réponse; faire très attention si jouer, tester le multi

    let résultats:Vec<(String,String)>;

    noms.insert(0,mon_nom.clone());

    let mut jeux = Jeux::nouveau(mode.clone(), joueur.clone(), liste, nb_manche);

    jeux.jouer(if option { Some(info) } else { None });

    résultats = met_a_jour_les_résultats(&mut sockets,&joueur).await;

    afficher_résultat(nb_client,&noms,mon_nom,&résultats);

    partage_résultat(&mut sockets,résultats,noms).await;
}

async fn envoi_message_tous(sockets: &mut Vec<TcpStream>, message: &String) {
    for socket in sockets {
        envoie_message(socket, message.clone()).await;
    }
}


async fn envoi_jeux(sockets: &mut Vec<TcpStream>, mode: Mode, liste: Vec<(String,String)>){
    for socket in sockets {
        let longeur = liste.len();
        let jeux = Jeux::nouveau(mode.clone(), Joueur::nouveau(), liste.clone(), longeur);
        let jeux_string = serde_json::to_string(&jeux).unwrap();
        envoie_message(socket,jeux_string).await;
    }
}



async fn met_a_jour_les_résultats(sockets :&mut Vec<TcpStream>,moi:&Joueur) -> Vec<(String,String)> {
    let mut résultats:Vec<(String,String)> = Vec::new();
    for mut socket in sockets {
        let buffer = lis_buffer(&mut socket).await.unwrap();
        let mut itérateur = buffer.splitn(2,";");
        let bonne_réponse = itérateur.next().unwrap();
        let mauvaise_réponse = itérateur.next().unwrap();
        let résultat = (bonne_réponse.to_string(),mauvaise_réponse.to_string());
        résultats.push(résultat);
    }
    résultats = ajoute_mes_résultats(résultats, moi);
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


async fn partage_résultat(sockets: &mut Vec<TcpStream>,résultats:Vec<(String,String)>, noms :Vec<String>){
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
    for mut socket in sockets {
        envoie_message(&mut socket,message.clone()).await;
    }
}


fn demander_nb_joueur() -> usize {
    afficher_str("Pour combien de joueur ? (hormis toi)");
    loop {
        let nb_joueur = demander();

        if nb_joueur.parse::<i32>().is_ok(){
            return nb_joueur.parse::<i32>().unwrap() as usize;
        }
    }
}


async fn lis_buffer(socket:&mut TcpStream) -> Result<String,Box<dyn std::error::Error>>{
    let mut buffer = vec![0u8; 1024];
    let n = socket.read(&mut buffer).await?;
    if n == 0 {
        return Err("Le client a fermé la connexion".into());
    }
    let littérale = String::from_utf8_lossy(&buffer[..n]).to_string();
    Ok(littérale)

}


async fn envoie_message(socket:&mut TcpStream, message:String){
    let message = message + "\n";
    let message_bytes = message.as_bytes();
    socket.write_all(message_bytes).await.unwrap();
}


async fn connextion_au_client(nb_client: usize) -> Result<(Vec<String>,Vec<TcpStream>), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:9000").await?;
    let mut noms_joueurs = Vec::new();
    let mut sockets = Vec::new();
    for _ in 0..nb_client {
        if nb_client > 1 {
            afficher_str("En attente des joueurs...");
        }else{
            afficher_str("En attente du joueur...");
        }

        let (mut socket, adresse) = listener.accept().await?;
        afficher(format!("Client connecté : {}", adresse));

        let nom = lis_buffer(&mut socket).await?;

        noms_joueurs.push(nom);
        sockets.push(socket);
    }

    Ok((noms_joueurs,sockets))
}