async function details(){
        const mode = document.querySelector('input[name="mode"]:checked');
          if (mode.value === "classique"){
                document.getElementById('nombre-de-manche').style.display = 'block';
              document.getElementById('nombre-seconde').style.display = 'none';
          }
          else if (mode.value === "chronomètre"){
              document.getElementById('nombre-de-manche').style.display = 'none';
              document.getElementById('nombre-seconde').style.display = 'block';
          }
          else if (mode.value === "survie"){
              document.getElementById('nombre-de-manche').style.display = 'none';
              document.getElementById('nombre-seconde').style.display = 'none';
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
                console.log("je suis seul");
                document.getElementById("role").style.display = "none";
                document.getElementById("nom").style.display = "none";
                document.getElementById("selection-mode-jeu").style.display = "block";
            }
            else {
                console.log("je ne suis pas seul");
                const role = document.querySelector('input[name="role"]:checked');
                if (role.value === "client"){
                    document.getElementById("selection-mode-jeu").style.display = "none";
                }
                document.getElementById("nom").style.display = "block";
                document.getElementById("role").style.display = "block";
            }

          } else {
            //document.getElementById("selection-mode-jeu").style.display = "none";
          }
      }

  async function hello() {
    console.log("hello");
    const response = await window.__TAURI__.core.invoke("say_hello", {
      name: document.getElementById("name").value
    });

    document.getElementById("result").textContent = response;
  }

  async function lancer(){
      console.log("lancer");
      const multi = document.querySelector('input[name="multi_non"]:checked').value;
      const nom = document.getElementById("nom").value;
      if (multi === "multi") {
          if (nom === "") {
              document.getElementById("impossible").textContent = "⚠️ Il vous faut un nom pour continuer";
              return;
          }
      }
      document.getElementById("page-paramètre").style.display = "none";
      document.getElementById("page-jeu").style.display = "block";


      const modeSelection = document.querySelector('input[name="mode"]:checked').value;
      let details_;  // Déclare la variable details

          // Si le mode est "classique"
          if (modeSelection === "classique") {
              details_ = document.getElementById("manche");
          }
          // Si le mode est "chronomètre"
          else if (modeSelection === "chronomètre") {
              details_ = document.getElementById("seconde");
          }



    console.log("on m'a appelé");
      await window.__TAURI__.core.invoke("commence", {
          multi: document.querySelector('input[name="multi_non"]:checked'),
          mode: document.querySelector('input[name="mode"]:checked'),
          details: details_,
          role: document.querySelector('input[name="role"]:checked'),
          nom: document.getElementById("nom").value
      });
  }

      async function commence() {
     // Affiche la “page-paramètre” et cache la page d’accueil
    document.getElementById("page-accueil").style.display = "none";
    document.getElementById("page-paramètre").style.display = "block";
  }


  async function max_manche(){
      try {
          const max_manche = await window.__TAURI__.core.invoke("nombre_question");
          console.log("Valeur de max_manche:", max_manche);
          return max_manche;
      } catch (error) {
          console.error("Erreur lors de l'appel Tauri:", error);
          return 75;
      }
  }

  async function modifie_max(){
    const max = await max_manche();
      console.log("max_manche dans modifie_max:", max);
    document.getElementById("manche").max = max;
      document.getElementById("manche").value = 10;
    document.getElementById("valeur_manche").textContent = "10 manches";
  }

  async function change_manche(){
    const valeur = document.getElementById("manche");
      document.getElementById("valeur_manche").textContent = valeur.value + " manches";
  }

    async function change_secondes(){
        const valeur = document.getElementById("seconde");
        document.getElementById("valeur_seconde").textContent = valeur.value + " secondes";
    }

    async function role_f(){
        const role = document.querySelector('input[name="role"]:checked').value;
        console.log(role);
        if (role === "hote") {
            document.getElementById("selection-mode-jeu").style.display = "block";
            const mode = document.querySelector('input[name="selection-mode-jeu"]:checked').value;
            if (mode === "classique" || mode === ""){
                document.getElementById("nombre-de-manche").style.display = "block";
            }
        } else if (role === "client") {
            document.getElementById("selection-mode-jeu").style.display = "none";
            console.log("bonjour");
        }

    }



document.addEventListener("DOMContentLoaded", modifie_max);
