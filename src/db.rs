use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::env;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(
        env::var("DATABASE_URL").expect("DATABASE_URL")
    );
    let size = env::var("POOL_SIZE").expect("POOL_SIZE");
    let u_size = size.parse::<u32>().expect("Not parse pool size");
    Pool::builder().max_size(u_size)
        .build(manager)
        .expect("Error with build poll connection")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);


impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
