mod task;


fn main() {
    let mut taches: Vec<task::Task> = Vec::new();
    taches.push(task::Task {id: 1, title: "premier".to_string(), done: false});
    taches.push(task::Task {id: 2, title: "second".to_string(), done: false});
    taches.push(task::Task {id: 3, title: "troisieme".to_string(), done: false});
    for i in &taches {
        println!("{} {} {}", i.id, i.title, i.done)
    }
    //println!("{}", &taches[0]);
}
