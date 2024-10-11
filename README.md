# <i>Bibliothèque Rust</i>

<i>Un système simple de gestion de bibliothèque implémenté en Rust.</i>

## <i>Fonctionnalités</i>

- <i>Ajouter de nouveaux livres à la bibliothèque</i>
- <i>Emprunter des livres</i>
- <i>Retourner des livres empruntés</i>
- <i>Afficher les livres disponibles</i>
- <i>Rechercher des livres par titre</i>

## <i>Structure du projet</i>

Le projet est composé de deux fichiers principaux :

- `src/main.rs` : Contient la logique principale du programme et l'interface utilisateur.
- `src/library.rs` : Définit les structures `Book` et `Library` ainsi que leurs implémentations.

## <i>Installation</i>

1. Assurez-vous d'avoir Rust installé sur votre système. Si ce n'est pas le cas, vous pouvez l'installer depuis [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clonez ce dépôt :
   ```
   git clone https://github.com/votre-nom-utilisateur/bibliotheque-rust.git
   cd bibliotheque-rust
   ```

3. Compilez le projet :
   ```
   cargo build --release
   ```

## <i>Utilisation</i>

Pour lancer le programme, exécutez :
   ```
   cargo run --release
   ```

Suivez les instructions à l'écran pour interagir avec la bibliothèque.

## <i>Accessibilité</i>

Ce projet a été conçu en gardant à l'esprit les principes d'accessibilité :

- <i>Interface utilisateur textuelle simple et claire</i>
- <i>Messages d'erreur explicites</i>
- <i>Compatibilité avec les lecteurs d'écran grâce à des sorties textuelles bien structurées</i>

## <i>Contribution</i>

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request.

## <i>Licence</i>

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
