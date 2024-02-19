# mps_project

Microbe platform system - project manager

1. docker-compose up postgres # para criar todos os bancos
2. cp crates/mps_project/config.toml.sample crates/mps_project/config.toml
3. cargo run -p mps_project -- migration --path crates/mps_project/migrations/ --config crates/mps_project/config.toml
4. cargo run -p mps_project -- seed --size 10 --config crates/mps_project/config.toml
5. cargo run -p mps_project -- grpc_server --config crates/mps_project/config.toml

## CreateProject

```console
grpcurl -plaintext -d '{"user_id":"12345678-1234-1234-1234-123456789abc","name":"Project Name","description":"Project Description"}' localhost:50051 proto.ProjectCrud/CreateProject
```

## ReadProject

```console
grpcurl -plaintext -d '{"id":"12345678-1234-1234-1234-123456789abc"}' localhost:50051 proto.ProjectCrud/ReadProject
```

## UpdateProject

```console
grpcurl -plaintext -d '{"id":"12345678-1234-1234-1234-123456789abc","user_id":"12345678-1234-1234-1234-123456789abc","name":"Updated Project Name","description":"Updated Project Description"}' localhost:50051 proto.ProjectCrud/UpdateProject
```

## DeleteProject


```console
grpcurl -plaintext -d '{"id":"12345678-1234-1234-1234-123456789abc"}' localhost:50051 proto.ProjectCrud/DeleteProject
```

## ToDo

* [ ] environment
