use crate::models::NewRoute;
use crate::schema::routes;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert(repo: &Repo, route: NewRoute) -> Result<usize, Error> {
    let route = diesel::insert_into(routes::table)
        .values(&route)
        .execute(&repo.conn())?;
    Ok(route)
}
