use chess::chess_service_client::ChessServiceClient;
use chess::MoveRequest;

pub mod chess {
    tonic::include_proto!("chess");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = ChessServiceClient::connect("http://[::1]:50051").await?;

    let request: tonic::Request<_> = tonic::Request::new(MoveRequest { from: 8, to: 16 });

    let response = client.move_piece(request).await?;
    println!("Move success: {}", response.into_inner().success);

    Ok(())
}
