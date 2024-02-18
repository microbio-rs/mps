# mps_project

Microbe platform system - project manager

1. docker-compose up postgres # para criar todos os bancos
2. cp crates/mps_project/config.toml.sample crates/mps_project/config.toml
3. cargo run -p mps_project -- migration --path crates/mps_project/migrations/ --config crates/mps_project/config.toml
4. cargo run -p mps_project -- seed --size 10 --config crates/mps_project/config.toml
5. cargo run -p mps_project -- grpc_server --config crates/mps_project/config.toml
