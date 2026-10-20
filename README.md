# Mot
README généré le: 20/10/26.    
Dernière mise à jour: 20/10/26.    

## Description
Mot est un petit jeu terminal dont le but est de trouver un mot depuis ça définition.
Il se joue seul ou à plusieurs.

## Installation

### Imot
Pour l’installer il est possible d’utiliser [imot](https://github.com/Lefevres/imot).

### Home-manager
Pour installer ```mot``` avec home-manager il faut utiliser une dérivation. Elle vous est déjà fournit dans ```mot.nix```. Pour l’utiliser il faut la déplacer dans le home-manager:
```
cp mot.nix ~/.config/home-manager/mot.nix
```
Puis l’appeler depuis ```home.nix``` avec :
```
{ pkgs, ... }:
let 
    mot = pkgs.callPackage ./mot.nix {};
in 
{
    home.packages = [
        mot
    ];
}
```
Ensuite home-manager râlera à cause des hash qui ne correspondent pas, il suffit de modifier les ```lib.fakeHash``` par ceux qui vous sont donnés.

### Façon classique
Sinon il est possible de l’installer manuellement, pour cela il faut le cloner:
```
git clone https://github.com/Lefevres/mot.git
```

puis le compiler:
```
mv mot 
cargo build --release
```

Mot utilise par défaut le dossier ```.mot``` dans lequel devrait se trouver ```mot.txt```.

Pour cela:
```
mkdir -p .mot
cp mot.txt ~/.mot/mot.txt
```

Ensuite il suffit d’exécuter le binaire.
Pour cela, un simple ```cargo run --release``` suffit. Sinon le binaire se trouve dans ```mot/target/release/mot```.


## Variable d’environnement
Mot utilise une variable d’environnement:
```
EMPLACEMENT_MOT
```

Elle sert à dire a mot où trouver ```mot.txt```, par défaut il le cherche a ```~/.mot```.



## Dépendance
Mot utilise cargo, il le faudra donc pour compiler mot, pas mais pas pour l’exécuter. Il peut être installé avec:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
