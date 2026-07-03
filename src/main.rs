mod task;


fn main() {
    let tache = task::Task {id: 1, title: "premiere tache".to_string(), done: false};
    println!("{} {} {}", tache.id, tache.title, tache.done);
}
