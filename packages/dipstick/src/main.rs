use std::process::ExitCode;

use anyhow::Context;
use tokio::sync::mpsc;
use tonic::transport::Server;

use self::can::Can;
use crate::core::Core;
use crate::xcp::Xcp;

mod can;
mod consts;
mod core;
mod xcp;

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

    let core = Core::new(log_handle, shutdown_tx);
    let can = Can::new();
    let xcp = Xcp::new();

    let reflection_server = reflection_server_builder()
        .build()
        .context("build reflection server")?;

    let addr = "127.0.0.1:3000".parse()?;
    tracing::debug!("starting server at {addr:?}");
    let res = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(reflection_server))
        .add_service(tonic_web::enable(core.into_server()))
        .add_service(tonic_web::enable(can.into_server()))
        .add_service(tonic_web::enable(xcp.into_server()))
        .serve_with_shutdown(addr, async {
            shutdown_rx.recv().await;
        })
        .await
        .map_err(anyhow::Error::from);

    tracing::info!("shutting down");

    // TODO: wait for all tasks to finish

    res
}

fn reflection_server_builder() -> tonic_reflection::server::Builder<'static> {
    let mut builder = tonic_reflection::server::Builder::configure();
    for file_descriptor_set in &[
        dipstick_proto::can::v1::FILE_DESCRIPTOR_SET,
        dipstick_proto::core::v1::FILE_DESCRIPTOR_SET,
        dipstick_proto::isotp::v1::FILE_DESCRIPTOR_SET,
        dipstick_proto::uds::v1::FILE_DESCRIPTOR_SET,
        dipstick_proto::wkt::FILE_DESCRIPTOR_SET,
        dipstick_proto::xcp::v1::FILE_DESCRIPTOR_SET,
    ] {
        builder = builder.register_encoded_file_descriptor_set(file_descriptor_set);
    }
    builder
}

#[tokio::main]
async fn main() -> ExitCode {
    match _main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("FATAL: {err:?}");
            ExitCode::FAILURE
        }
    }
}
