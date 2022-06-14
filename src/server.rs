use futures::Stream;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

use hello::hello_server::{Hello, HelloServer};
use hello::{HelloRequest, HelloResponse};
use learning_grpc::hello;

use dotenv::dotenv;
use downloader::downloader_server::{Downloader, DownloaderServer};
use downloader::{
    CancelRequest, DataChunk, DownloadRequest, DownloadResponse, DownloadResult, GetRequest,
};
use learning_grpc::downloader;
use log::{debug, error, info};

#[derive(Default)]
pub struct DownloaderService {}

#[tonic::async_trait]
impl Hello for DownloaderService {
    async fn hello_world(
        &self,
        _: Request<HelloRequest>,
    ) -> Result<Response<HelloResponse>, Status> {
        let response = HelloResponse {
            message: "Hello, World!".to_string(),
        };
        Ok(Response::new(response))
    }
}

#[tonic::async_trait]
impl Downloader for DownloaderService {
    type DownloadStream =
        Pin<Box<dyn Stream<Item = Result<DownloadResponse, Status>> + Send + Sync + 'static>>;
    type GetStream = Pin<Box<dyn Stream<Item = Result<DataChunk, Status>> + Send + Sync + 'static>>;

    async fn get(
        &self,
        _request: Request<GetRequest>,
    ) -> Result<Response<Self::GetStream>, Status> {
        debug!("Got a request: {:?}", _request);

        let (tx, rx) = mpsc::channel(4);

        //let planets: Vec<u8> = vec![0,0,0];

        tokio::spawn(async move {
            //let mut stream = tokio_stream::iter(&planets);

            tx.send(Ok(DataChunk {
                data: vec![0, 1, 2],
            }))
            .await
            .unwrap();

            // while let Some(planet) = stream.next().await {
            //     tx.send(Ok(DataChunk {
            //         data: vec![planet.clone()],
            //     }))
            //     .await
            //     .unwrap();
            // }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn download(
        &self,
        _request: Request<DownloadRequest>,
    ) -> Result<Response<Self::DownloadStream>, Status> {
        debug!("Got a request: {:?}", _request);

        let url= _request.into_inner().software_artifact.unwrap().url;
        debug!("Got a request url: {:?}", url);

        let (tx, rx) = mpsc::channel(4);

        //let planets: Vec<u8> = vec![0,0,0];

        tokio::spawn(async move {
            //let mut stream = tokio_stream::iter(&planets);

            tx.send(Ok(DownloadResponse {
                ..Default::default()
            }))
            .await
            .unwrap();

            // while let Some(planet) = stream.next().await {
            //     tx.send(Ok(DataChunk {
            //         data: vec![planet.clone()],
            //     }))
            //     .await
            //     .unwrap();
            // }
        });

        Ok(Response::new(Box::pin(ReceiverStream::new(rx))))
    }

    async fn cancel_download(
        &self,
        _request: Request<CancelRequest>,
    ) -> Result<Response<DownloadResult>, Status> {
        debug!("Got a request: {:?}", _request);

        let da= _request.into_inner().operation_id;

        let response = DownloadResult {
            code: 0,
            downloaded_artifact: "downloaded artifact".to_string(),
            text_message: "text message".to_string(),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    info!("Starting downloader server");

    let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    let srv = DownloaderService::default();
    Server::builder()
        //.add_service(HelloServer::new(srv))
        .add_service(DownloaderServer::new(srv))
        .serve(addr)
        .await?;

    Ok(())
}
