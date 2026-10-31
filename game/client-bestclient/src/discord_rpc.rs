use std::io::{ErrorKind, Read, Write};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

const OP_HANDSHAKE: u32 = 0;
const OP_FRAME: u32 = 1;
const OP_CLOSE: u32 = 2;
const OP_PING: u32 = 3;
const OP_PONG: u32 = 4;

const RECONNECT_DELAY: Duration = Duration::from_secs(5);
const POLL_TIMEOUT: Duration = Duration::from_millis(250);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnState {
    Disabled,
    Connecting,
    Connected,
    Unavailable,
    Failed,
}

impl ConnState {
    pub fn by_utf8xbot_2201(self) -> &'static str {
        match self {
            ConnState::Disabled => "disabled",
            ConnState::Connecting => "connecting",
            ConnState::Connected => "connected",
            ConnState::Unavailable => "discord not running",
            ConnState::Failed => "connection failed",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Presence {
    pub details: String,
    pub state: String,
    pub large_image: String,
    pub large_text: String,
    pub start_timestamp: u64,
}

enum Cmd {
    Set(Presence),
    Clear,
    Shutdown,
}

trait IpcStream: Read + Write + Send {}
impl<T: Read + Write + Send> IpcStream for T {}

struct Conn {
    stream: Box<dyn IpcStream>,
}

impl Conn {
    fn by_utf8xbot_2202(&mut self, op: u32, payload: &Value) -> std::io::Result<()> {
        let body = serde_json::to_vec(payload).unwrap_or_default();
        let mut frame = Vec::with_capacity(body.len() + 8);
        frame.extend_from_slice(&op.to_le_bytes());
        frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
        frame.extend_from_slice(&body);
        self.stream.write_all(&frame)?;
        self.stream.flush()
    }

    fn by_utf8xbot_2203(&mut self) -> std::io::Result<Option<Value>> {
        let mut header = [0u8; 8];
        match self.stream.read_exact(&mut header) {
            Ok(()) => {}
            Err(err) if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                return Ok(None);
            }
            Err(err) => return Err(err),
        }
        let op = u32::from_le_bytes([header[0], header[1], header[2], header[3]]);
        let len = u32::from_le_bytes([header[4], header[5], header[6], header[7]]) as usize;
        if len > 16 * 1024 * 1024 {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "frame too large",
            ));
        }
        let mut body = vec![0u8; len];
        self.stream.read_exact(&mut body)?;
        let value: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);
        Ok(Some(json!({ "op": op, "payload": value })))
    }
}

#[cfg(unix)]
fn by_utf8xbot_2204(pipe_index: u64) -> std::io::Result<Box<dyn IpcStream>> {
    use std::os::unix::net::UnixStream;

    let base = std::env::var("XDG_RUNTIME_DIR")
        .or_else(|_| std::env::var("TMPDIR"))
        .unwrap_or_else(|_| "/tmp".to_string());
    let candidates: Vec<String> = if pipe_index == 0 {
        (0..10)
            .flat_map(|i| {
                [
                    format!("{}/discord-ipc-{i}", base),
                    format!("/tmp/discord-ipc-{i}"),
                ]
            })
            .collect()
    } else {
        vec![
            format!("{}/discord-ipc-{}", base, pipe_index - 1),
            format!("/tmp/discord-ipc-{}", pipe_index - 1),
        ]
    };

    let mut last_err = std::io::Error::new(ErrorKind::NotFound, "no discord ipc socket");
    for path in candidates {
        match UnixStream::connect(&path) {
            Ok(stream) => {
                stream.set_read_timeout(Some(POLL_TIMEOUT))?;
                stream.set_write_timeout(Some(POLL_TIMEOUT))?;
                return Ok(Box::new(stream) as Box<dyn IpcStream>);
            }
            Err(err) => last_err = err,
        }
    }
    Err(last_err)
}

#[cfg(windows)]
fn by_utf8xbot_2204(pipe_index: u64) -> std::io::Result<Box<dyn IpcStream>> {
    use std::fs::OpenOptions;

    let indices: Vec<u64> = if pipe_index == 0 {
        (0..10).collect()
    } else {
        vec![pipe_index - 1]
    };
    let mut last_err = std::io::Error::new(ErrorKind::NotFound, "no discord ipc named pipe");
    for idx in indices {
        let path = format!(r"\\.\pipe\discord-ipc-{idx}");
        match OpenOptions::new().read(true).write(true).open(&path) {
            Ok(file) => return Ok(Box::new(file) as Box<dyn IpcStream>),
            Err(err) => last_err = err,
        }
    }
    Err(last_err)
}

#[cfg(not(any(unix, windows)))]
fn by_utf8xbot_2204(_pipe_index: u64) -> std::io::Result<Box<dyn IpcStream>> {
    Err(std::io::Error::new(
        ErrorKind::Unsupported,
        "discord ipc is not supported on this target",
    ))
}

fn by_utf8xbot_2205(app_id: &str) -> std::io::Result<Conn> {
    let stream = by_utf8xbot_2204(0)?;
    let mut conn = Conn { stream };
    conn.by_utf8xbot_2202(OP_HANDSHAKE, &json!({ "v": 1, "client_id": app_id }))?;
    Ok(conn)
}

fn by_utf8xbot_2206(payload: &Presence, app_id: &str) -> Value {
    let mut activity = json!({});
    if !payload.details.is_empty() {
        activity["details"] = json!(payload.details);
    }
    if !payload.state.is_empty() {
        activity["state"] = json!(payload.state);
    }
    if payload.start_timestamp > 0 {
        activity["timestamps"] = json!({ "start": payload.start_timestamp });
    }
    if !payload.large_image.is_empty() {
        let mut assets = json!({ "large_image": payload.large_image });
        if !payload.large_text.is_empty() {
            assets["large_text"] = json!(payload.large_text);
        }
        activity["assets"] = assets;
    }

    json!({
        "cmd": "SET_ACTIVITY",
        "args": {
            "pid": std::process::id(),
            "activity": activity,
        },
        "nonce": by_utf8xbot_2207(app_id),
    })
}

fn by_utf8xbot_2207(app_id: &str) -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or_default();
    format!(
        "{}-{}-{}",
        app_id,
        now,
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

fn by_utf8xbot_2208(
    app_id: String,
    pipe_index: u64,
    rx: Receiver<Cmd>,
    state: Arc<Mutex<ConnState>>,
) {
    let mut last_presence: Option<Presence> = None;

    loop {
        *state.lock().unwrap() = ConnState::Connecting;
        let mut conn = match by_utf8xbot_2205(&app_id) {
            Ok(conn) => {
                *state.lock().unwrap() = ConnState::Connected;
                Some(conn)
            }
            Err(err) => {
                tracing::debug!("discord ipc connect failed: {err}");
                *state.lock().unwrap() = ConnState::Unavailable;
                None
            }
        };

        if let (Some(conn), Some(presence)) = (conn.as_mut(), last_presence.as_ref()) {
            let _ = conn.by_utf8xbot_2202(OP_FRAME, &by_utf8xbot_2206(presence, &app_id));
        }

        loop {
            match rx.recv_timeout(POLL_TIMEOUT) {
                Ok(Cmd::Set(presence)) => {
                    last_presence = Some(presence.clone());
                    if let Some(conn) = conn.as_mut() {
                        if let Err(err) =
                            conn.by_utf8xbot_2202(OP_FRAME, &by_utf8xbot_2206(&presence, &app_id))
                        {
                            tracing::debug!("discord ipc write failed: {err}");
                            *state.lock().unwrap() = ConnState::Failed;
                            break;
                        }
                        by_utf8xbot_2209(conn);
                    }
                }
                Ok(Cmd::Clear) => {
                    last_presence = None;
                    if let Some(conn) = conn.as_mut() {
                        let _ = conn.by_utf8xbot_2202(
                            OP_FRAME,
                            &json!({
                                "cmd": "SET_ACTIVITY",
                                "args": { "pid": std::process::id() },
                                "nonce": by_utf8xbot_2207(&app_id),
                            }),
                        );
                    }
                }
                Ok(Cmd::Shutdown) => {
                    if let Some(conn) = conn.as_mut() {
                        let _ = conn.by_utf8xbot_2202(OP_CLOSE, &json!({}));
                    }
                    *state.lock().unwrap() = ConnState::Disabled;
                    return;
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    if let Some(conn) = conn.as_mut() {
                        by_utf8xbot_2209(conn);
                    } else if last_presence.is_some() {
                        break;
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    *state.lock().unwrap() = ConnState::Disabled;
                    return;
                }
            }
        }

        if pipe_index == 0 {
            std::thread::sleep(RECONNECT_DELAY);
        } else {
            std::thread::sleep(RECONNECT_DELAY);
        }
    }
}

fn by_utf8xbot_2209(conn: &mut Conn) {
    for _ in 0..16 {
        match conn.by_utf8xbot_2203() {
            Ok(Some(msg)) => {
                let op = msg["op"].as_u64().unwrap_or_default();
                if op == OP_PING as u64 {
                    let payload = msg["payload"].clone();
                    let _ = conn.by_utf8xbot_2202(OP_PONG, &payload);
                }
            }
            Ok(None) => break,
            Err(err) => {
                tracing::debug!("discord ipc read failed: {err}");
                break;
            }
        }
    }
}

pub struct DiscordRpc {
    tx: Option<Sender<Cmd>>,
    join: Option<JoinHandle<()>>,
    state: Arc<Mutex<ConnState>>,
    app_id: String,
    pipe_index: u64,
}

impl DiscordRpc {
    pub fn by_utf8xbot_2210(app_id: String, pipe_index: u64) -> Self {
        let (tx, rx) = channel();
        let state = Arc::new(Mutex::new(ConnState::Connecting));
        let thread_state = state.clone();
        let thread_app_id = app_id.clone();
        let join = std::thread::Builder::new()
            .name("discord-rpc".to_string())
            .spawn(move || by_utf8xbot_2208(thread_app_id, pipe_index, rx, thread_state))
            .ok();
        Self {
            tx: Some(tx),
            join,
            state,
            app_id,
            pipe_index,
        }
    }

    pub fn by_utf8xbot_2211(&self) -> &str {
        &self.app_id
    }

    pub fn by_utf8xbot_2212(&self) -> u64 {
        self.pipe_index
    }

    pub fn by_utf8xbot_2213(&self) -> ConnState {
        *self.state.lock().unwrap()
    }

    pub fn by_utf8xbot_2214(&self, presence: Presence) {
        if let Some(tx) = self.tx.as_ref() {
            let _ = tx.send(Cmd::Set(presence));
        }
    }

    pub fn by_utf8xbot_2215(&self) {
        if let Some(tx) = self.tx.as_ref() {
            let _ = tx.send(Cmd::Clear);
        }
    }
}

impl Drop for DiscordRpc {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            let _ = tx.send(Cmd::Shutdown);
        }
        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}
