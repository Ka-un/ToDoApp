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
    todo::ToDoList::print(&taches);

    //indique qu'une tache est complété
    todo::ToDoList::complete(&mut taches, 1);

    //affiche la liste des taches
    todo::ToDoList::print(&taches);

    todo::ToDoList::remove(&mut taches, 3);

    todo::ToDoList::print(&taches);
}

