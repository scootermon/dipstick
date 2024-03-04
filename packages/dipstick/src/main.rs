use std::process::ExitCode;

use anyhow::Context;
use tokio::sync::mpsc;
use tonic::transport::Server;

use crate::core::Core;

mod consts;
mod core;

async fn _main() -> anyhow::Result<()> {
    let log_handle = core::logging::init();

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);
    tokio::spawn({
        let shutdown_tx = shutdown_tx.clone();
        async move {
            if let Err(err) = tokio::signal::ctrl_c().await {
                tracing::error!("failed to register signal handler for ctrl-c: {err}");
                return;
            }

            tracing::info!("received ctrl-c");
            let _ = shutdown_tx.send(()).await;
        }
    });

    let server = Core::new(log_handle, shutdown_tx);
    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(dipstick_proto::can::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::core::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::isotp::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::uds::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::wkt::FILE_DESCRIPTOR_SET)
        .build()
        .context("build gRPC reflection server")?;

    let addr = "127.0.0.1:3000".parse()?;
    tracing::debug!("starting server at {addr:?}");
    let res = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(reflection_server))
        .add_service(tonic_web::enable(server.into_server()))
        .serve_with_shutdown(addr, async {
            shutdown_rx.recv().await;
        })
        .await
        .map_err(anyhow::Error::from);

    tracing::info!("shutting down");

    // TODO: wait for all tasks to finish

    res
}

#[tokio::main]
async fn main() -> ExitCode {
    match _main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err:?}");
            ExitCode::FAILURE
        }
    }
}
