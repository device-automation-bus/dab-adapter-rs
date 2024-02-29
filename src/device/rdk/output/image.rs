use crate::dab::structs::CaptureScreenshotRequest;
use crate::dab::structs::CaptureScreenshotResponse;
use crate::dab::structs::DabError;
use crate::device::rdk::interface::http_post;
use crate::device::rdk::interface::{service_activate, get_service_state};
use serde::{Deserialize, Serialize};

use base64::{engine::general_purpose, Engine as _};
use bytes::Bytes;
use hyper::server::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use hyper::{Method, StatusCode};
use local_ip_address::local_ip;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration};

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn process(_dab_request: CaptureScreenshotRequest) -> Result<String, DabError> {
    //######### Activate org.rdk.ScreenCapture #########
    if get_service_state("org.rdk.ScreenCapture")? != "activated" {
        service_activate("org.rdk.ScreenCapture".to_string())?;
        thread::sleep(Duration::from_millis(500));
    }

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut ResponseOperator = CaptureScreenshotResponse::default();
        // *** Fill in the fields of the struct CaptureScreenshotResponse here ***

        //######### Enable the Http server #########
        let my_local_ip = local_ip().unwrap();
        let my_server: String =
            "http://".to_string() + &my_local_ip.to_string() + &":7878/upload".to_string();
        let addr = SocketAddr::from(([0, 0, 0, 0], 7878));
        let (tx, mut rx) = mpsc::channel(10);

        let make_svc = make_service_fn(move |_conn| {
            let tx = tx.clone();
            async move { Ok::<_, Infallible>(service_fn(move |req| handle_req(req, tx.clone()))) }
        });

        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        let server = Server::bind(&addr).serve(make_svc);
        let graceful = server.with_graceful_shutdown(async {
            shutdown_rx.await.ok();
        });

        tokio::spawn(graceful);

        //#########org.rdk.ScreenCapture.uploadScreenCapture#########
        #[derive(Serialize)]
        struct UploadScreenCaptureRequest {
            jsonrpc: String,
            id: i32,
            method: String,
            params: UploadScreenCaptureRequestParams,
        }

        #[derive(Serialize)]
        struct UploadScreenCaptureRequestParams {
            url: String,
            callGUID: String,
        }

        let req_params = UploadScreenCaptureRequestParams {
            url: my_server,
            callGUID: "12345".to_string(),
        };

        let request = UploadScreenCaptureRequest {
            jsonrpc: "2.0".into(),
            id: 3,
            method: "org.rdk.ScreenCapture.uploadScreenCapture".into(),
            params: req_params,
        };

        #[derive(Deserialize)]
        struct UploadScreenCaptureResponse {
            jsonrpc: String,
            id: i32,
            result: UploadScreenCaptureResult,
        }

        #[derive(Deserialize)]
        struct UploadScreenCaptureResult {
            success: bool,
        }

        let json_string = serde_json::to_string(&request).unwrap();
        http_post(json_string)?;

        //######### Listen for the base64 string from the request handler with a timeout. #########
        match time::timeout(Duration::from_secs(30), rx.recv()).await {
            Ok(Some(data)) => {
                let b64 = general_purpose::STANDARD.encode(&data);
                let b64 = format!("data:image/png;base64,{}", b64);
                // After receiving the base64 string, signalize to close the server.
                let _ = shutdown_tx.send(());

                //######### Correlate Fields #########
                ResponseOperator.outputImage = b64;

                // *******************************************************************
                Ok(serde_json::to_string(&ResponseOperator).unwrap())
            }
            Ok(None) => Err(DabError::Err500(
                "The channel was closed before a message was received".to_string(),
            )),
            Err(_) => Err(DabError::Err500(
                "Timed out waiting for a message from the channel".to_string(),
            )),
        }
    })
}

async fn handle_req(
    req: Request<Body>,
    tx: mpsc::Sender<Bytes>,
) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/upload") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();

            if tx.send(whole_body).await.is_err() {
                return Ok(Response::new(Body::from("Error processing the request")));
            }
            Ok(Response::new(Body::from("File processed successfully")))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}
