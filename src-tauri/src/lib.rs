use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Bytes, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use serde_json::json;
use http_body_util::Full;

#[derive(Clone, Debug)]
struct HeartBeatData {
    value: i32,
    timestamp: u64,
}

static HEART_BEAT_DATA: std::sync::LazyLock<Arc<Mutex<Option<HeartBeatData>>>> = 
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(None)));

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn update_heart_beat(heart_rate: i32) -> Result<String, String> {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("Time error: {}", e))?
        .as_secs();
    
    let heart_data = HeartBeatData {
        value: heart_rate,
        timestamp: current_time,
    };
    
    match HEART_BEAT_DATA.lock() {
        Ok(mut data) => {
            *data = Some(heart_data);
            Ok("Heart beat updated successfully".to_string())
        }
        Err(e) => Err(format!("Failed to update heart beat: {}", e))
    }
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

async fn handle_request(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let response = match req.uri().path() {
        "/" => {
            let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>BLE Heart Beat Server</title>
    <meta charset="utf-8">
</head>
<body>
    <div class="heartbeat-container">
        <p class="heartbeat-count" id="heartbeatCount">--</p>
    </div>

    <script>
        function updateHeartRate() {
            fetch('/api/heart')
                .then(response => response.json())
                .then(data => {
                    const countElement = document.getElementById('heartbeatCount');
                    
                    if (data.heart_beat !== null && data.heart_beat !== undefined) {
                        countElement.textContent = data.heart_beat;
                        countElement.className = 'heartbeat-count connected';
                    } else {
                        countElement.textContent = '--';
                        countElement.className = 'heartbeat-count disconnected';
                    }
                })
                .catch(error => {
                    const countElement = document.getElementById('heartbeatCount');
                    countElement.textContent = '--';
                    countElement.className = 'heartbeat-count error';
                });
        }
        setInterval(updateHeartRate, 1000);
        updateHeartRate();
    </script>
</body>
</html>
"#;
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(Full::new(Bytes::from(html)))
                .unwrap()
        }
        "/api/heart" => {
            let current_time = get_current_timestamp();
            let heart_beat_value = match HEART_BEAT_DATA.lock() {
                Ok(data) => {
                    match data.as_ref() {
                        Some(heart_data) => {
                            if current_time - heart_data.timestamp > 30 {
                                None
                            } else {
                                Some(heart_data.value)
                            }
                        }
                        None => None
                    }
                }
                Err(_) => None
            };
            
            let response_data = json!({
                "heart_beat": heart_beat_value
            });
            
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .body(Full::new(Bytes::from(response_data.to_string())))
                .unwrap()
        }
        _ => {
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::from("Not Found")))
                .unwrap()
        }
    };

    Ok(response)
}

async fn start_http_server() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 25872));
    let listener = TcpListener::bind(addr).await.expect("Failed to bind to address");
    
    println!("HTTP Server running on http://localhost:25872");

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept connection");
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    
    rt.spawn(start_http_server());
    
    rt.block_on(async {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![greet, update_heart_beat])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
