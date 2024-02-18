use std::net::SocketAddr;

use tonic::{transport::Server, Request, Response, Status};

use payments::payment_server::{Payment, PaymentServer};
use payments::{PaymentRequest, PaymentResponse};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct PaymentsService {}

#[tonic::async_trait]
impl Payment for PaymentsService {
    async fn send_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Got a request: {:?}", request);

        let req = request.into_inner();

        let response = PaymentResponse {
            successful: true,
            message: format!("Payment of {} sent successfully to {}", req.amount, req.to),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse()?;
    let payment_service = PaymentsService::default();

    Server::builder()
        .add_service(PaymentServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}
