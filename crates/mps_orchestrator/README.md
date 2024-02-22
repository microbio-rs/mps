# mps_orchestration

Microbe platform system - orchestration manager

## apply resource

example input:

```json
{"context": "{\"name\": \"mps-sample-nestjs\", \"port\": 3000, \"image\":\"${account_id}.dkr.ecr.us-east-1.amazonaws.com/mps-sample-nestjs\", \"version\":\"0.0.1\", \"namespace\": \"platform-engineering\", \"replicas\": 2, \"domain\": \"dev\"}"}
```

```console
grpcurl -plaintext -import-path ./crates/mps_orchestrator/proto -proto kubernetes.proto -d '{"context": "{\"name\": \"mps-sample-nestjs\", \"port\": 3000, \"image\":\"${account_id}.dkr.ecr.us-east-1.amazonaws.com/mps-sample-nestjs\", \"version\":\"0.0.1\", \"namespace\": \"platform-engineering\", \"replicas\": 2, \"domain\": \"dev\"}"}' '[::1]:50060' kubernetes_proto.Kubernetes/Apply
```

## minikube (k8s local)

### addons

```console
minikube addons configure registry-creds # ecr
minikube addons enable registry-creds
minikube addons enable ingress
```

