use car::car_service_server::CarService;
use car::{Car, CreateCarRequest, CreateCarResponse, SearchCarRequest, SearchCarResponse};
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

use crate::db_connection;

pub mod car {
    tonic::include_proto!("route.car");
}

#[derive(Debug)]
pub struct RouteCarService {
    cars: Arc<Vec<Car>>,
}

#[tonic::async_trait]
impl CarService for RouteCarService {
    type SearchCarStream =
        Pin<Box<dyn Stream<Item = Result<SearchCarResponse, Status>> + Send + Sync + 'static>>;
    async fn create_car(
        &self,
        request: Request<CreateCarRequest>,
    ) -> Result<Response<CreateCarResponse>, Status> {
        let pool = db_connection::pg_pool_handler().map_err(|e| e.into());
    }
    async fn search_car(
        &self,
        request: Request<SearchCarRequest>,
    ) -> Result<Response<Self::SearchCarStream>, Status> {
    }
}
