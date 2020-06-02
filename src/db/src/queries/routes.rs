use crate::models::{NewRoute, Route};
use crate::schema::routes;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert(repo: &Repo, route: NewRoute) -> Result<Route, Error> {
    let route = diesel::insert_into(routes::table)
        .values(&route)
        .get_result::<Route>(&repo.conn())?;
    Ok(route)
}
