use diesel::r2d2::{ConnectionManager, PooledConnection};

use super::Database;

pub trait DatabaseConnectable {
    fn get_connection(
        &self,
        db: &mut Database,
    ) -> &mut PooledConnection<ConnectionManager<diesel::PgConnection>>;
    //  {
    //     //db.connection.as_mut().unwrap()
    // }
}
