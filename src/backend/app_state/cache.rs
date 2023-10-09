pub struct Connection {
    pub address: String,
    pub password: String,
    pub port: u16,
}

impl Connection {
    fn connect(&self) -> String {
        format!("redis://:{}@{}:{}", self.password, self.address, self.port)
    }
}

pub struct Cache {
    pub client: redis::Client,
}

impl Cache {
    pub fn new(connection: Connection) -> Cache {
        let params = connection.connect();
        let client = redis::Client::open(params).unwrap();
        Cache { client }
    }

    pub fn default() -> Cache {
        Self::new(Connection {
            address: "127.0.0.1".to_string(),
            password: "==PaSsWoRd==".to_string(),
            port: 6379,
        })
    }
}
