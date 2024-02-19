pub mod proto {
    tonic::include_proto!("project_proto");
    // tonic::include_proto!("application_proto");
}

#[cfg(feature = "grpc_server")]
pub mod server;
#[cfg(feature = "grpc_server")]
pub use server::*;

#[cfg(feature = "grpc_client")]
pub mod client;
