async function details(){
        const mode = document.querySelector('input[name="mode"]:checked');
          if (mode.value === "classique"){
                document.getElementById('nombre-de-manche').style.display = 'block';
          }
          else{
              //document.getElementById('nombre-de-manche').style.display = 'none';  //chronomètre
          }
      }

      async function en_tete(){

      }
      async function cache(){
      document.querySelectorAll('*').forEach(function(element) {
              element.style.display = 'none';
          });
      }

      async function confirmer(){


          //document.getElementById("selection-mode-jeu").style.display = "block";

          const reponse = document.querySelector('input[name="multi_non"]:checked');
          if (reponse) {
            if (reponse.value === "seul"){
                document.getElementById("selection-mode-jeu").style.display = "block";
            }
            else {
                document.getElementById("selection-mode-jeu").style.display = "none";
            }

          } else {
            //document.getElementById("selection-mode-jeu").style.display = "none";
          }
      }

  async function hello() {
    const response = await window.__TAURI__.core.invoke("say_hello", {
      name: document.getElementById("name").value
    });

    document.getElementById("result").textContent = response;
  }

      async function commence() {
     // Affiche la “page-paramètre” et cache la page d’accueil
    document.getElementById("page-accueil").style.display = "none";
    document.getElementById("page-paramètre").style.display = "block";
  }


  async function reponse(){}
  async function max_manche(){
      try {
          const max_manche = await window.__TAURI__.core.invoke("nombre_question");
          console.log("Valeur de max_manche:", max_manche);  // Afficher la valeur dans le terminal
          return max_manche;
      } catch (error) {
          console.error("Erreur lors de l'appel Tauri:", error);
          return 75;  // Valeur par défaut en cas d'erreur
      }
  }

  async function modifie_max(){
    const max = await max_manche();
      console.log("max_manche dans modifie_max:", max);
    document.getElementById("manche").max = max; // Appliquer la valeur à l'attribut max du slider
      document.getElementById("manche").value = 10;
    document.getElementById("valeur_manche").textContent = "Valeur : 10"; //
  }

  async function change_manche(){
    const valeur = document.getElementById("manche");
      document.getElementById("valeur_manche").textContent = "Valeur : "+valeur.value;
  }
document.addEventListener("DOMContentLoaded", modifie_max);