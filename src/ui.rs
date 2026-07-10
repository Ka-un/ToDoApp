use std::io;
use crate::todo;


pub fn afficher_menu() {
        println!("uuuuuuuuuu MENU uuuuuuuuuu");
        println!("1) Afficher la ToDoList");
        println!("2) Ajouter une tâche");
        println!("3) Indiquer qu'une tâche est terminée");
        println!("4) supprimer une tâche");
        println!("uuuuuuuuuuuuuuuuuuuuuuuuuu");
}

//permet à l'utilisateur de supprimer une tache depuis le menu
pub fn remove_tache(to_do_list: &mut todo::ToDoList) {
    let mut input_id: String = String::new();
    println!("Veuillez rentrer l'id de la tache à supprimer");
    io::stdin().read_line(&mut input_id).unwrap();
    let input_id: u32 = match input_id.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Ce n'est pas un nombre valide.");
            return
        },
    };
    match todo::ToDoList::remove(to_do_list, input_id) {
        Ok(()) => println!("La tache n° {} a été supprimée", input_id),
        Err(e) => println!("{}", e),
    }
}

pub fn ajouter_tache(to_do_list: &mut todo::ToDoList) {
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

pub fn complete_tache(to_do_list: &mut todo::ToDoList) {
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
    match todo::ToDoList::complete(to_do_list, input_id) {
        Ok(()) => println!("La tache n° {} est terminée", input_id),
        Err(e) => println!("{}", e),
    }
}