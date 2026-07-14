//use mysql::prelude::*;
use mysql::*;

pub fn connexion() -> Result<PooledConn, mysql::Error> {
    let url = "mysql://ToDo:353232@localhost:3306/todolist";
    let opts = Opts::from_url(url)?;
    let pool = Pool::new(opts)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}