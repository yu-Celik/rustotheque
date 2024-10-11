mod library;
use library::{Book, Library};
use std::io;

fn main() {
    let mut library: Library = Library::new();

    library.add_book(Book::new_book(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        false,
    ));
    library.add_book(Book::new_book(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        false,
    ));
    library.add_book(Book::new_book(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        false,
    ));
    library.add_book(Book::new_book(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        false,
    ));
    library.add_book(Book::new_book(
        "1984".to_string(),
        "George Orwell".to_string(),
        1949,
        false,
    ));
    library.add_book(Book::new_book(
        "Le Petit Prince".to_string(),
        "Antoine de Saint-Exupéry".to_string(),
        1943,
        false,
    ));
    library.add_book(Book::new_book(
        "Dune".to_string(),
        "Frank Herbert".to_string(),
        1965,
        false,
    ));

    loop {
        println!("\nQue souhaitez-vous faire ? Typez un nombre :");
        println!("1. Emprunter un livre");
        println!("2. Retourner un livre");
        println!("3. Afficher les livres disponibles");
        println!("4. Rechercher un livre");
        println!("5. Ajouter un nouveau livre");
        println!("6. Quitter");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Échec de la lecture de l'entrée");

        match choice.trim() {
            "1" => {
                println!("Entrez le titre du livre à emprunter :");

                let mut titre = String::new();
                io::stdin()
                    .read_line(&mut titre)
                    .expect("Échec de la lecture de l'entrée");

                let titre = titre.trim();

                if !library.borrow_book(titre) {
                    println!("Désolé, le livre '{}' n'est pas disponible.", titre);
                    continue;
                }
                println!("Livre '{}' emprunté avec succès.", titre);
            }
            "2" => {
                println!("Entrez le titre du livre à retourner :");
                let mut titre = String::new();

                io::stdin()
                    .read_line(&mut titre)
                    .expect("Échec de la lecture de l'entrée");

                let titre = titre.trim();

                if !library.return_book(titre) {
                    println!(
                        "Erreur : le livre '{}' n'était pas emprunté ou n'existe pas.",
                        titre
                    );
                    continue;
                }
                println!("Livre '{}' retourné avec succès.", titre);
            }
            "3" => {
                library.display_available_books();
            }
            "4" => {
                println!("Veuillez saisir le titre du livre à rechercher :");

                let mut titre = String::new();

                io::stdin()
                    .read_line(&mut titre)
                    .expect("Échec de la lecture de l'entrée");

                let titre = titre.trim();

                let result = library.search_book(titre);

                if result.is_none() {
                    println!("Aucun livre trouvé");
                    continue;
                }
            }
            "5" => {
                println!("Entrez le titre du livre :");
                let mut titre = String::new();
                io::stdin()
                    .read_line(&mut titre)
                    .expect("Échec de la lecture du titre");
                let titre = titre.trim().to_string();

                println!("Entrez l'auteur du livre :");
                let mut auteur = String::new();
                io::stdin()
                    .read_line(&mut auteur)
                    .expect("Échec de la lecture de l'auteur");
                let auteur = auteur.trim().to_string();

                println!("Entrez l'année de création :");
                let mut annee = String::new();
                io::stdin()
                    .read_line(&mut annee)
                    .expect("Échec de la lecture de l'année");
                let annee: u32 = annee
                    .trim()
                    .parse()
                    .expect("Veuillez entrer un nombre valide");

                let nouveau_livre = Book::new_book(titre.clone(), auteur, annee, false);
                library.add_book(nouveau_livre);
                println!("Le livre '{}' a été ajouté avec succès.", titre);
            }
            "6" => {
                println!("Merci d'avoir utilisé notre bibliothèque. Au revoir !");
                break;
            }
            _ => println!("Choix non valide. Veuillez réessayer."),
        }
    }
}
