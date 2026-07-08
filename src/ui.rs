use std::io;
use crate::todo;

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