//structure des taches
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

//struture de la liste de toutes les taches
pub struct ToDoList {
    pub tasks : Vec<Task>
}

//fonctions lié à la liste des taches
impl ToDoList {
    //ajoute une tache
    pub fn add(&mut self, t: Task) {
        self.tasks.push(t);
    }

    //affiche toutes les taches
    pub fn print(&self) {
        println!("-----");
        for i in &self.tasks {
        println!("{} {} {}", i.id, i.title, i.done);
        }
        println!("-----");
    }

    //indique qu'une tache est terminée
    pub fn complete(&mut self , id: u32){
        for i in &mut self.tasks {
            if i.id == id {
                i.done = true;
                break
            }
        }
    }
}