# Bibliothèque Rust

Un système simple de gestion de bibliothèque implémenté en Rust.

## Fonctionnalités

- Ajouter de nouveaux livres à la bibliothèque
- Emprunter des livres
- Retourner des livres empruntés
- Afficher les livres disponibles
- Rechercher des livres par titre

## Structure du projet

Le projet est composé de deux fichiers principaux :

- `src/main.rs` : Contient la logique principale du programme et l'interface utilisateur.
- `src/library.rs` : Définit les structures `Book` et `Library` ainsi que leurs implémentations.

## Installation

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

## Utilisation

Pour lancer le programme, exécutez :
   ```
   cargo run --release
   ```

Suivez les instructions à l'écran pour interagir avec la bibliothèque.

## Accessibilité

Ce projet a été conçu en gardant à l'esprit les principes d'accessibilité :

- Interface utilisateur textuelle simple et claire
- Messages d'erreur explicites
- Compatibilité avec les lecteurs d'écran grâce à des sorties textuelles bien structurées

## Contribution

Les contributions sont les bienvenues ! N'hésitez pas à ouvrir une issue ou à soumettre une pull request.

## Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.
