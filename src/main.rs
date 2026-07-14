use std::io;

mod todo;
mod ui;
mod database;


fn main() {

    let mut conn = match database::connexion() {
       Ok(conn) => {
            println!("Connexion réussie à la base de donnée");
            conn
        }
        Err(e) => {
            println!("Erreur : {}", e);
            return;
        }
    };

    //création de la liste des taches
    let mut taches = todo::ToDoList::new();

    println!("Chargement de la database");
    match database::load_todolist(&mut taches, &mut conn) {
        Ok(_) => println!("Chargement terminé"),

        Err(e) => println!("Erreur chargement : {}", e),
    }



    loop {
        ui::afficher_menu();

        let input: u32 = match ui::lecture_input().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            //TODO : remplacer par une fonction ui
            1 => todo::ToDoList::print(&taches),
            2 => ui::ajouter_tache(&mut taches),
            3 => ui::complete_tache(&mut taches),
            4 => ui::remove_tache(&mut taches),
            5 => {
                ui::quit_app(&mut taches, &mut conn);
                break;
            }
            _ => continue,
        }

        println!("Appuyez sur Entrée pour continuer...");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        ////clear le terminal à chaque boucle 
        print!("\x1B[2J\x1B[1;1H");
    }
}

