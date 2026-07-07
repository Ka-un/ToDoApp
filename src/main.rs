use std::io;

mod todo;


fn main() {

    //création des taches
    let mut taches = todo::ToDoList {
        tasks: Vec::new(),
    };
    //todo::ToDoList::print(&taches);
    //todo::ToDoList::add(&mut taches, todo::Task {id: 1, title: "premier".to_string(), done: false});
    // todo::ToDoList::add(&mut taches, todo::Task {id: 2, title: "second".to_string(), done: false});
    // todo::ToDoList::add(&mut taches, todo::Task {id: 3, title: "troisieme".to_string(), done: false});

    todo::ToDoList::complete(&mut taches, 1);

    todo::ToDoList::remove(&mut taches, 1);

    loop {
        println!("uuuuuuuuuu MENU uuuuuuuuuu");
        println!("1) Afficher la ToDoList");
        println!("2) Ajouter une tâche");
        println!("3) Indiquer qu'une tâche est terminée");
        println!("4) supprimer une tâche \n");

        println!("uuuuuuuuuuuuuuuuuuuuuuuuuu");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match input {
            1 => todo::ToDoList::print(&taches),
            2 => {
                    let mut input_id: String = String::new();
                    let mut input_title: String = String::new();

                    /////Bloque pour passer input_id de String à u32
                    println!("Veuillez rentrer un id pour la nouvelle tâche");
                    io::stdin().read_line(&mut input_id).unwrap();
                    let input_id: u32 = match input_id.trim().parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Ce n'est pas un nombre valide.");
                            println!("Appuyez sur Entrée pour continuer...");

                            let mut _buffer = String::new();
                            io::stdin().read_line(&mut _buffer).unwrap();
                            continue
                        },
                    };
                    println!("Vous avez rentré le nombre {}", input_id);
                    /////
                    
                    println!("Veuillez rentrer un titre pour la nouvelle tâche");
                    io::stdin().read_line(&mut input_title).unwrap();

                    todo::ToDoList::add(&mut taches, todo::Task {id: input_id, title: input_title, done: false});
                },
            _ => continue,
        }

        println!("Appuyez sur Entrée pour continuer...");

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        ////clear le terminal à chaque boucle 
        print!("\x1B[2J\x1B[1;1H");
    }
}

