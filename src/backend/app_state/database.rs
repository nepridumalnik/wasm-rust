use mysql::prelude::Queryable;

const CREATE_MAIN_TABLE: &str = r"CREATE TABLE IF NOT EXISTS Users (
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

pub struct Connection {
    pub name: String,
    pub password: String,
    pub address: String,
    pub port: u16,
}

impl Connection {
    fn connect(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/main_database",
            self.name, self.password, self.address, self.port
        )
    }
}

pub struct Database {
    pub pool: mysql::Pool,
}

impl Database {
    pub fn new(connection: Connection) -> Database {
        let connect = connection.connect();
        let pool = mysql::Pool::new(connect.as_str()).unwrap();
        let db = Database { pool };

        let mut conn = db.pool.get_conn().unwrap();
        conn.query_drop(CREATE_MAIN_TABLE).unwrap();

        db
    }

    pub fn default() -> Database {
        Self::new(Connection {
            name: "root".to_string(),
            password: "==PaSsWoRd==".to_string(),
            address: "localhost".to_string(),
            port: 3306,
        })
    }
}
