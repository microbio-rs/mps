#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() {
    init_tracing();
    println!("hello mps");
}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
    registry().with(fmt::layer()).with(EnvFilter::from_env("MPS_LOG")).init();
}
