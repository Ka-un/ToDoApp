use std::io;

mod todo;


fn main() {

    //création de la liste des taches
    let mut taches = todo::ToDoList {
        tasks: Vec::new(),
    };

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
                ajouter_tache(&mut taches);
                },
            3 => {
                complete_tache(&mut taches);
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

fn ajouter_tache(to_do_list: &mut todo::ToDoList) {
    let mut input_id: String = String::new();
    let mut input_title: String = String::new();

    /////Bloque pour passer input_id de "String" à "u32"
    println!("Veuillez rentrer un id pour la nouvelle tâche");
    io::stdin().read_line(&mut input_id).unwrap();
    let input_id: u32 = match input_id.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Ce n'est pas un nombre valide.");
            return
            },
        };
    println!("Vous avez rentré le nombre {}", input_id);
    /////
                    
    println!("Veuillez rentrer un titre pour la nouvelle tâche");
    io::stdin().read_line(&mut input_title).unwrap();

    todo::ToDoList::add(to_do_list, todo::Task {id: input_id, title: input_title, done: false});
}

fn complete_tache(to_do_list: &mut todo::ToDoList) {
    let mut input_id: String = String::new();
    println!("Veuillez rentrer l'id de la tâche terminée");
    io::stdin().read_line(&mut input_id).unwrap();
    let input_id: u32 = match input_id.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Ce n'est pas un nombre valide.");
            return
        },
    };
    todo::ToDoList::complete(to_do_list, input_id);
    println!("La tache n° {} est terminée", input_id);

}

