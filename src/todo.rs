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
}