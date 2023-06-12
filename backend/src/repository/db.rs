use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

type DBPool = Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new(database_url: String) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    // pub fn get_todos(&self) -> Vec<Todo> {
    //     todos
    //         .load::<Todo>(&mut self.pool.get().unwrap())
    //         .expect("Error loading all todos")
    // }
}
