use mysql::prelude::Queryable;

const URL: &str = "mysql://root:==PaSsWoRd==@localhost:3306/main_database";

const CREATE_TABLE: &str = r"CREATE TABLE IF NOT EXISTS Payments(
    id int not null,
    amount int not null,
    name text
)";

pub struct Connection {
    pool: mysql::Pool,
}

pub struct Payments {
    pub id: u32,
    pub amount: u32,
    pub name: String,
}

pub struct PaymentsTable {
    pub conn: mysql::PooledConn,
}

impl Connection {
    pub fn new() -> Connection {
        Connection {
            pool: mysql::Pool::new(URL).unwrap(),
        }
    }

    pub fn get_pool(&self) -> &mysql::Pool {
        return &self.pool;
    }
}

impl PaymentsTable {
    pub fn new(pool: &mysql::Pool) -> PaymentsTable {
        let mut table = PaymentsTable {
            conn: pool.get_conn().unwrap(),
        };

        table.conn.query_drop(CREATE_TABLE).unwrap();

        return table;
    }
}
