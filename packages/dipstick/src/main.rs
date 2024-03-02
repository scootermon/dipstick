use std::process::ExitCode;

use anyhow::Context;
use tonic::transport::Server;

mod consts;
mod server;

async fn _main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:3000".parse()?;

    let server = server::Server::new();
    let reflection_server = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(dipstick_proto::can::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::server::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::uds::FILE_DESCRIPTOR_SET)
        .register_encoded_file_descriptor_set(dipstick_proto::wkt::FILE_DESCRIPTOR_SET)
        .build()
        .context("build gRPC reflection server")?;

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
