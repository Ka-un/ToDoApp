mod todo;


fn main() {

    //création des taches
    let mut taches = todo::ToDoList {
        tasks: Vec::new(),
    };
    todo::ToDoList::add(&mut taches, todo::Task {id: 1, title: "premier".to_string(), done: false});
    todo::ToDoList::add(&mut taches, todo::Task {id: 2, title: "second".to_string(), done: false});
    todo::ToDoList::add(&mut taches, todo::Task {id: 3, title: "troisieme".to_string(), done: false});

    //affichages des taches
    println!("-----");
    for i in &taches.tasks {
        println!("{} {} {}", i.id, i.title, i.done);
    }
    println!("-----");

    //modification d'une tache
    for i in &mut taches.tasks {
        if i.id == 1 {
            i.done = true;
        }
    }

    println!("-----");
    for i in &taches.tasks {
        println!("{} {} {}", i.id, i.title, i.done);
    }
    println!("-----");
}

