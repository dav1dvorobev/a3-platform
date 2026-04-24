#[tokio::main]
async fn main() {
    let manifest = a3_manifest::Manifest::from_path("agent.json").unwrap();
    a3_runtime::serve(manifest).await.unwrap();
}
