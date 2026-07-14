use mysql::prelude::*;
use mysql::*;

use crate::todo::*;

pub fn connexion() -> Result<PooledConn, mysql::Error> {
    let url = "mysql://ToDo:353232@localhost:3306/todolist";
    let opts = Opts::from_url(url)?;
    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}

pub fn load_todolist(todolist: &mut ToDoList, conn: &mut PooledConn) ->  Result<(), mysql::Error>{
    let result: Vec<(u32, String, bool)> = conn.query(
        "SELECT id, titre, done FROM taches"
        )?;
    
    for (id, title, done) in result {
        todolist.tasks.push(Task {
            id,
            title,
            done,
        });
    };

    Ok(())    
}

pub fn save_todolist(todolist: &mut ToDoList, conn: &mut PooledConn) -> Result<(), mysql::Error>{
    for task in 0..todolist.tasks.len() {
        let id = todolist.tasks[task].id;
        let title = &todolist.tasks[task].title;
        let done = todolist.tasks[task].done;

        let existe: bool = conn.exec_first(
            "SELECT EXISTS(SELECT 1 FROM taches WHERE id = ?)",
            (id,)
        )?
            .unwrap_or(false);

        if existe {
            conn.exec_drop(
                "UPDATE taches 
                 SET titre = ?, done = ?
                 WHERE id = ?",
                (title, done, id)
            )?;
        } else {
            conn.exec_drop(
                "INSERT INTO taches (id, titre, done) VALUES (?, ?, ?)",
                (id, title, done),
            )?;
        }
    }
    Ok(())
}