# mps

microbe platform system

```rust
// Cloud
let credential = AwsCredential::default()
  .access_key("access_key")
  .access_secret("access_secret")
  .iam_role("arn::role")
  .build()?;
let cloud = CloudBuilder::default()
  .provider(CloudProvider::Aws)
  .credential(credential)
  .build()?;

// Organization
let org = OrgBuilder::default()
  .name("myorg")
  .cloud(cloud)
  .project(
    // Project
    let project = ProjectBuilder::default()
      .name("myproject")
      .description("my super project")
      .env(
        // Environment
        EnvBuilder::default()
          .name("development")
          .description("development environment")
          .services(
            // Service
            ServiceBuilder::default()
              .name("my service")
              .build()?
          )
          .build()?
      )
      .build()?
  )
  .build()?;

```

or


```rust
#[derive(Serialize, Deserialize)]
struct Context<'a> {
  name: Cow<'a, str>
}

let (rx, tx) = mpsc::channe();

let service = ServiceBuilder::default()
  .template_clone("https//github.com/microbio-rs/mps-sample-nestjs")
  .out_to("/tmp/project")
  .checkout("c3080f2")
  .context(Context{
    name: Cow::from("myproject")
  })
  .render_to("/tmp/project_new")
  .docker_file("Dockerfile.prod")
  .docker_path(".")
  .kube_local();
  .debug(tx)
  .build()?;

service
  .deploy();
  // .push_ecr()
  // .apply()
  // .send();
```
