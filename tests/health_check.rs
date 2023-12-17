use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String{
    // using port 0 to trigger an OS scan for any available port to bind
    let lst = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    
    let port = lst.local_addr().unwrap().port();

    let server = actix_backend::run(lst).expect("Failed to bind address");
    
    let _ = tokio::spawn(server);

    // return application address to the caller 
    format!("http://127.0.0.1:{}", port)
}
