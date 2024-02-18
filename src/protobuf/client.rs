// gRPC tut

use payments::{payment_client::PaymentClient, PaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(PaymentRequest {
        to: "Alice".to_owned(),
        amount: 420,
        from: "Pieter".to_owned(),
    });

    let response = client.send_payment(request).await?;

    println!("RESPONSE={:?}", response);
    Ok(())
}
