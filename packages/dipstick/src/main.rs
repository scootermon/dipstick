use std::process::ExitCode;

use anyhow::Context;
use tonic::transport::Server;

use crate::core::Core;

mod consts;
mod core;

async fn _main() -> anyhow::Result<()> {
    let log_handle = core::logging::init();

    let addr = "127.0.0.1:3000".parse()?;

    let server = Core::new(log_handle);
    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(dipstick_proto::can::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::core::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::isotp::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::uds::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::wkt::FILE_DESCRIPTOR_SET)
        .build()
        .context("build gRPC reflection server")?;

    tracing::debug!("Starting server at {addr:?}");
    Server::builder()
        .accept_http1(true)
        .add_service(reflection_server)
        .add_service(tonic_web::enable(server.into_server()))
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    match _main().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err:?}");
            ExitCode::FAILURE
        }
    }
}
