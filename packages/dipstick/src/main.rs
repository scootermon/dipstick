use std::process::ExitCode;
use std::sync::Arc;

use anyhow::Context;
use tokio::sync::mpsc;
use tonic::transport::Server;

mod consts;

async fn _main() -> anyhow::Result<()> {
    let log_handle = dipstick_core::logging::init();

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

    let reflection_v1_server = reflection_v1_server_builder()
        .build()
        .context("failed to build reflection v1 server")?;
    let reflection_v1alpha_server = reflection_v1alpha_server_builder()
        .build()
        .context("failed to build reflection v1alpha server")?;

    let core = dipstick_core::Core::new(consts::VERSION.to_owned(), log_handle, shutdown_tx);
    let gpio = dipstick_gpio::Gpio::new(Arc::clone(&core));
    let can = dipstick_can::Can::new(Arc::clone(&core));
    let spi = dipstick_spi::Spi::new(Arc::clone(&core));
    let xcp_service = dipstick_xcp::XcpService::new(Arc::clone(&core));
    let shadow_service = dipstick_shadow::ShadowService::new(Arc::clone(&core));
    let stack_service = dipstick_stack::StackService::new(
        Arc::clone(&core),
        Arc::clone(&can),
        Arc::clone(&gpio),
        Arc::clone(&spi),
        Arc::clone(&xcp_service),
        Arc::clone(&shadow_service),
    );

    let addr = "0.0.0.0:3000".parse()?;
    tracing::info!("listening on {addr:?}");
    let res = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(can.into_server()))
        .add_service(tonic_web::enable(core.into_server()))
        .add_service(tonic_web::enable(gpio.into_server()))
        .add_service(tonic_web::enable(reflection_v1_server))
        .add_service(tonic_web::enable(reflection_v1alpha_server))
        .add_service(tonic_web::enable(shadow_service.into_server()))
        .add_service(tonic_web::enable(spi.into_server()))
        .add_service(tonic_web::enable(stack_service.into_server()))
        .add_service(tonic_web::enable(xcp_service.into_server()))
        .serve_with_shutdown(addr, async {
            shutdown_rx.recv().await;
        })
        .await
        .map_err(anyhow::Error::from);

    tracing::info!("shutting down");

    res
}

fn reflection_v1_server_builder() -> tonic_reflection::server::Builder<'static> {
    let mut builder = tonic_reflection::server::Builder::configure();
    for file_descriptor_set in dipstick_proto::ALL_FILE_DESCRIPTOR_SETS {
        builder = builder.register_encoded_file_descriptor_set(file_descriptor_set);
    }
    builder
}

fn reflection_v1alpha_server_builder() -> tonic_reflection::server::v1alpha::Builder<'static> {
    let mut builder = tonic_reflection::server::v1alpha::Builder::configure();
    for file_descriptor_set in dipstick_proto::ALL_FILE_DESCRIPTOR_SETS {
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
