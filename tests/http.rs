use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Deserialize;
use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
struct DailyCountsResponse {
    date: String,
    add_count: u64,
    sub_count: u64,
    net: i64,
}

struct TestServer {
    base_url: String,
    child: Child,
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

static TEST_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));
static SERVER: Lazy<Mutex<Option<Arc<TestServer>>>> = Lazy::new(|| Mutex::new(None));

#[cfg(unix)]
mod cleanup {
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::Once;

    static REGISTER: Once = Once::new();
    static PID: AtomicI32 = AtomicI32::new(0);

    pub fn register(pid: u32) {
        REGISTER.call_once(|| {
            PID.store(pid as i32, Ordering::SeqCst);
            unsafe {
                libc::atexit(on_exit);
            }
        });
    }

    extern "C" fn on_exit() {
        let pid = PID.load(Ordering::SeqCst);
        if pid > 0 {
            unsafe {
                libc::kill(pid, libc::SIGTERM);
            }
        }
    }
}

fn pick_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind random port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn unique_data_path() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("web_app_http_{}_{}.json", std::process::id(), nanos));
    path.to_string_lossy().to_string()
}

async fn wait_until_ready(base_url: &str) {
    let client = Client::new();
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        if let Ok(resp) = client.get(format!("{base_url}/api/today")).send().await {
            if resp.status().is_success() {
                return;
            }
        }
        if Instant::now() > deadline {
            panic!("server did not become ready");
        }
        sleep(Duration::from_millis(100)).await;
    }
}

async fn spawn_server() -> TestServer {
    let port = pick_free_port();
    let data_path = unique_data_path();
    let child = Command::new(env!("CARGO_BIN_EXE_web_app"))
        .env("PORT", port.to_string())
        .env("APP_DATA_PATH", data_path)
        .env("RUST_LOG", "info")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to spawn server");

    #[cfg(unix)]
    cleanup::register(child.id());

    let base_url = format!("http://127.0.0.1:{port}");
    wait_until_ready(&base_url).await;

    TestServer { base_url, child }
}

async fn shared_server() -> Arc<TestServer> {
    let mut guard = SERVER.lock().await;
    if let Some(server) = guard.as_ref() {
        return Arc::clone(server);
    }
    let server = Arc::new(spawn_server().await);
    *guard = Some(Arc::clone(&server));
    server
}

#[tokio::test]
async fn http_click_add_updates_today() {
    let _guard = TEST_LOCK.lock().await;
    let server = shared_server().await;
    let client = Client::new();

    let before: DailyCountsResponse = client
        .get(format!("{}/api/today", server.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/api/click", server.base_url))
        .json(&serde_json::json!({ "action": "add" }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let today: DailyCountsResponse = client
        .get(format!("{}/api/today", server.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(today.add_count, before.add_count + 1);
    assert_eq!(today.sub_count, before.sub_count);
    assert_eq!(today.net, before.net + 1);
    assert!(!today.date.is_empty());
}

#[tokio::test]
async fn http_click_sub_updates_today() {
    let _guard = TEST_LOCK.lock().await;
    let server = shared_server().await;
    let client = Client::new();

    let before: DailyCountsResponse = client
        .get(format!("{}/api/today", server.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let response = client
        .post(format!("{}/api/click", server.base_url))
        .json(&serde_json::json!({ "action": "sub" }))
        .send()
        .await
        .unwrap();
    assert!(response.status().is_success());

    let today: DailyCountsResponse = client
        .get(format!("{}/api/today", server.base_url))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    assert_eq!(today.add_count, before.add_count);
    assert_eq!(today.sub_count, before.sub_count + 1);
    assert_eq!(today.net, before.net - 1);
    assert!(!today.date.is_empty());
}
