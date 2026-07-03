pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

pub struct ToDoList {
    pub tasks : Vec<Task>
}

//self représente la ToDoList | tasks l'élément de la struct
impl ToDoList {
    pub fn add(&mut self, t: Task) {
        self.tasks.push(t);
    }

    pub fn print(&self) {
        println!("-----");
        for i in &self.tasks {
        println!("{} {} {}", i.id, i.title, i.done);
        }
        println!("-----");
    }
}