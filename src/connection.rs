use mysql::prelude::Queryable;

const URL: &str = "mysql://root:==PaSsWoRd==@localhost:3306/main_database";

const CREATE_TABLE: &str = r"CREATE TABLE IF NOT EXISTS Users (
    ID INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    Name VARCHAR(50) NOT NULL,
    SecondName VARCHAR(50) NOT NULL,
    Age INT NOT NULL,
    Male BOOLEAN NOT NULL,
    Interests TEXT NOT NULL,
    City VARCHAR(50) NOT NULL,
    Password VARCHAR(50) NOT NULL,
    Email VARCHAR(50) NOT NULL UNIQUE,
    INDEX (Name, Email)
    ) ENGINE=InnoDB CHARSET=utf8";

pub struct Database {
    pub pool: mysql::Pool,
}

impl Database {
    pub fn new() -> Database {
        let table = Database {
            pool: mysql::Pool::new(URL).unwrap(),
        };

        let mut conn = table.pool.get_conn().unwrap();

        conn.query_drop(CREATE_TABLE).unwrap();

        return table;
    }
}
