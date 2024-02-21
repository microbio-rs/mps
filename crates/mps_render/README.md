# mps_render

Microbe platform system - render

```console
grpcurl -plaintext -import-path ./crates/mps_render/proto -proto render.proto -d '{"input": "/tmp/neide", "context": "{\"project_name\": \"ZUMBAAA\"}"}' '[::1]:50060' template_proto.Template/Render
```

