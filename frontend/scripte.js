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
    const max_manche = await window.__TAURI__.core.invoke("nombre_question()");
  }

  async function modifie_max(){
    const max = await max_manche();
    document.getElementById("manche").max = max;
  }
document.addEventListener("DOMContentLoaded", setSliderMax);