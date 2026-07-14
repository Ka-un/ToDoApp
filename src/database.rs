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

pub fn load_taches(todolist: &mut ToDoList, conn: &mut PooledConn) ->  Result<(), mysql::Error>{
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