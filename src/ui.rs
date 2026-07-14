use std::io;
use crate::todo;
//Gère les fonctions liés au menu 

pub fn afficher_menu() {
        println!("uuuuuuuuuu MENU uuuuuuuuuu");
        println!("1) Afficher la ToDoList");
        println!("2) Ajouter une tâche");
        println!("3) Indiquer qu'une tâche est terminée");
        println!("4) supprimer une tâche");
        println!("uuuuuuuuuuuuuuuuuuuuuuuuuu");
}

//Demande a l'utilisateur de rentrer une valeur 
fn lecture_input() -> String {
    let mut input_id: String = String::new();
    io::stdin()
        .read_line(&mut input_id)
        .unwrap();
    input_id
}

//permet à l'utilisateur de supprimer une tache depuis le menu
pub fn remove_tache(to_do_list: &mut todo::ToDoList) {
    println!("Veuillez rentrer l'id de la tache à supprimer");
    let input_id: u32 = match lecture_input()
                                        .trim()
                                        .parse() {
        Ok(n) => n,
        Err(e) => {
            println!("\n Error '{e}' \n\n Ce n'est pas un nombre valide. \n Veuillez réessayer \n");
            return
        },
    };
    match todo::ToDoList::remove(to_do_list, input_id) {
        Ok(()) => println!("La tache n° {} a été supprimée", input_id),
        Err(e) => println!("{}", e),
    }
}

//Permet à l'utilisateur d'ajouter une tache depuis le menu
pub fn ajouter_tache(to_do_list: &mut todo::ToDoList) {
    println!("Veuillez rentrer un id pour la nouvelle tâche");
    //Passe un String reçu en input en u32
    let input_id: u32 = match lecture_input()
                                        .trim()
                                        .parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Ce n'est pas un nombre valide.");
            return
            },
        };
    println!("Vous avez rentré le nombre {}", input_id);
    //
                    
    println!("Veuillez rentrer un titre pour la nouvelle tâche");
    let input_title = lecture_input();

    todo::ToDoList::add(to_do_list, todo::Task {id: input_id, title: input_title, done: false});
}

//Permet d'indiquer qu'une tache est terminée
pub fn complete_tache(to_do_list: &mut todo::ToDoList) {
    println!("Veuillez rentrer l'id de la tâche terminée");
    let input_id: u32 = match lecture_input()
                                        .trim()
                                        .parse() {
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