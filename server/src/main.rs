use chess::chess_service_server::{ChessService, ChessServiceServer};
use chess::{MoveRequest, MoveResponse};
use shared::board::Board;
use shared::mov::Move;
use tonic::{transport::Server, Request, Response, Status};
use tonic_web::GrpcWebLayer;

pub mod chess {
    tonic::include_proto!("chess");
}

#[derive(Default)]
pub struct ChessServer {
    board: Board,
}

#[tonic::async_trait]
impl ChessService for ChessServer {
    async fn move_piece(
        &self,
        request: Request<MoveRequest>,
    ) -> Result<Response<MoveResponse>, Status> {
        let request = request.into_inner();
        let mv = Move {
            from: request.from as usize,
            to: request.to as usize,
        };

        let mut board = self.board.clone();
        let success = board.move_pawn(mv);

        Ok(Response::new(MoveResponse { success }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let chess_server = ChessServer::default();

    println!("Chess gRPC server listening on {}", addr);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(ChessServiceServer::new(chess_server))
        .serve(addr)
        .await?;

    Ok(())
}
