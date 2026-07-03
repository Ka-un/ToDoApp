mod todo;


fn main() {

    //création des taches
    let mut taches: Vec<todo::Task> = Vec::new();
    taches.push(todo::Task {id: 1, title: "premier".to_string(), done: false});
    taches.push(todo::Task {id: 2, title: "second".to_string(), done: false});
    taches.push(todo::Task {id: 3, title: "troisieme".to_string(), done: false});

    //affichages des taches
    println!("-----");
    for i in &taches {
        println!("{} {} {}", i.id, i.title, i.done);
    }
    println!("-----");

    //modification d'une tache
    for i in &mut taches {
        if i.id == 1 {
            i.done = true;
        }
    }

    println!("-----");
    for i in &taches {
        println!("{} {} {}", i.id, i.title, i.done);
    }
    println!("-----");
}

//fn add_todo(t: todo::Task) {
//}
