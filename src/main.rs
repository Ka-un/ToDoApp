use std::io;

mod todo;
mod ui;
mod database;


fn main() {

    match database::connexion() {
       Ok(_) => {
            println!("Connexion réussie à la base de donnée");
        }
        Err(e) => {
            println!("Erreur : {}", e);
        }
    }

    //création de la liste des taches
    let mut taches = todo::ToDoList::new();


    loop {
        ui::afficher_menu();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => todo::ToDoList::print(&taches),
            2 => ui::ajouter_tache(&mut taches),
            3 => ui::complete_tache(&mut taches),
            4 => ui::remove_tache(&mut taches),
            _ => continue,
        }

        println!("Appuyez sur Entrée pour continuer...");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        ////clear le terminal à chaque boucle 
        print!("\x1B[2J\x1B[1;1H");
    }
}

