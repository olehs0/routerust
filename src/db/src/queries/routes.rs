use crate::models::{NewRoute, Route};
use crate::schema::routes;
use crate::schema::routes::dsl::*;
use crate::Repo;
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert(repo: &Repo, route: NewRoute) -> Result<Route, Error> {
    println!("route {:?}", route);
    let route = diesel::insert_into(routes::table)
        .values(&route)
        .get_result::<Route>(&repo.conn())?;
    let routes_rows = routes.load::<Route>(&repo.conn());
    println!("routes_rows {:?}", routes_rows);
    Ok(route)
}
