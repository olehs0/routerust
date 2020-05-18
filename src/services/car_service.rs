use car::car_service_server::CarService;
use car::{Car, CreateCarRequest, CreateCarResponse, SearchCarRequest};
use futures::Stream;
use std::pin::Pin;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub mod car {
    tonic::include_proto!("route.car");
}

#[derive(Debug)]
pub struct RouteCarService {
    cars: Arc<Vec<Car>>,
}

#[tonic::async_trait]
impl CarService for RouteCarService {
    type SearchCarStream = Pin<Box<dyn Stream<Item = Result<Car, Status>> + Send + Sync + 'static>>;
    async fn create_car(
        &self,
        request: Request<CreateCarRequest>,
    ) -> Result<Response<CreateCarResponse>, Status> {
        let response = CreateCarResponse::default();
        return Ok(Response::new(response));
    }
    async fn search_car(
        &self,
        request: Request<SearchCarRequest>,
    ) -> Result<Response<Self::SearchCarStream>, Status> {
        let mut stream = request.into_inner();
        let output = async_stream::try_stream! {
            yield Car::default().clone()
        };
        Ok(Response::new(Box::pin(output) as Self::SearchCarStream))
    }
}
