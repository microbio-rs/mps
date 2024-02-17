pub mod scm {
    tonic::include_proto!("scm");
}

#[cfg(feature = "grpc_server")]
pub mod server;
#[cfg(feature = "grpc_server")]
pub use server::*;

#[cfg(feature = "grpc_client")]
pub mod client;
