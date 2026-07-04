use std::io;

mod todo;


fn main() {

    //création des taches
    let mut taches = todo::ToDoList {
        tasks: Vec::new(),
    };
    todo::ToDoList::print(&taches);
    todo::ToDoList::add(&mut taches, todo::Task {id: 1, title: "premier".to_string(), done: false});
    // todo::ToDoList::add(&mut taches, todo::Task {id: 2, title: "second".to_string(), done: false});
    // todo::ToDoList::add(&mut taches, todo::Task {id: 3, title: "troisieme".to_string(), done: false});

    todo::ToDoList::complete(&mut taches, 1);

    todo::ToDoList::remove(&mut taches, 1);

    loop {
        ////clear le terminal à chaque boucle 
        //print!("\x1B[2J\x1B[1;1H");

        println!("uuuuuuuuuu MENU uuuuuuuuuu");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!{"vous avez choisis {}", input}
    }

}

