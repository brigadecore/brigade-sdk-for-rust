# Brigade SDK for Rust

This is an experimental Rust SDK for Brigade 2, and is currently a work in progress.

Currently, all tests assume a port forwarding session has been started locally on port 8080:

```
kubectl port-forward services/brigade-apiserver 8080:443 -n brigade
```

### Example

```rust
    let address = "https://localhost:8080";
    let cfg = ClientConfig {
        allow_insecure_connections: true,
    };
    let sc = SessionsClient::new(String::from(address), cfg.clone(), None).unwrap();
    let token = sc
        .create_root_session("F00Bar!!!".to_string())
        .await
        .unwrap();

    let pc = ProjectsClient::new(String::from(address), cfg, Some(token.value)).unwrap();
    let p = pc.get("hello-world".to_string()).await.unwrap();
    println!("{:#?}", p);
```
