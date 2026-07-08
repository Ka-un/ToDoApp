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
            println!("id : {} titre : {} état : {}", i.id, i.title, i.done);
        }
        println!("-----");
    }

    //Retourne l'index d'une tâche en fonction de son id
    pub fn get_index(&self, id: u32) ->  Option<usize> {
        for i in 0..self.tasks.len() {
            if self.tasks[i].id == id {
               return Some(i);
            }
        }
        return None;
    }

    //indique qu'une tache est terminée
    pub fn complete(&mut self , id: u32) -> Result<(), String>{
        match self.get_index(id) {
            Some(index) => {
                self.tasks[index].done = true;
                Ok(())
            }
            None => {
                Err(format!("la tâche n° {} n'a pas pu être modifiée, l'id n'existe pas", id))
            }
        }
    }

    //supprime la tache avec un id précis
    pub fn remove(&mut self, id: u32){
        match self.get_index(id) {
            Some(index) => {
                self.tasks.remove(index);
            }
            None => {
                println!("la tâche n° {} n'a pas pu être supprimée, l'id n'existe pas", id)
            }
        }
    }
}