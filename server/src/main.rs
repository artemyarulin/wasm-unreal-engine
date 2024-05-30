use actix::prelude::*;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;
use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct FileChanged;

struct MyWebSocket {
    sessions: Arc<Mutex<Vec<Recipient<FileChanged>>>>,
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.sessions
            .lock()
            .unwrap()
            .push(ctx.address().recipient());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.sessions
            .lock()
            .unwrap()
            .retain(|addr| addr != &ctx.address().recipient());
    }
}

impl Handler<FileChanged> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, _: FileChanged, ctx: &mut Self::Context) {
        ctx.binary(vec![]);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg);
        }
    }
}

async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<Arc<Mutex<Vec<Recipient<FileChanged>>>>>,
) -> impl Responder {
    ws::start(
        MyWebSocket {
            sessions: data.get_ref().clone(),
        },
        &req,
        stream,
    )
}

fn run_file_watcher(sessions: Arc<Mutex<Vec<Recipient<FileChanged>>>>, path: PathBuf) {
    thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())
            .expect("Failed to create a file watcher");
        watcher
            .watch(path.as_path(), RecursiveMode::NonRecursive)
            .expect("Failed to start a file watcher");
        let debounce_time = Duration::from_millis(200);
        let mut last_event_time = Instant::now() - debounce_time;
        while let Ok(Ok(event)) = rx.recv() {
            if let EventKind::Modify(ModifyKind::Data(_)) = event.kind {
                let now = Instant::now();
                if now.duration_since(last_event_time) >= debounce_time {
                    println!("File changed: {:?}", event);
                    let sessions = sessions.lock().unwrap();
                    for addr in sessions.iter() {
                        addr.do_send(FileChanged);
                    }
                    last_event_time = now;
                }
            }
        }
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sessions = Arc::new(Mutex::new(Vec::new()));
    run_file_watcher(
        sessions.clone(),
        Path::new("../client-web/core/core_bg.wasm").to_path_buf(),
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sessions.clone()))
            .route("/ws", web::get().to(websocket_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
