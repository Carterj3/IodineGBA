use ::backtrace::Backtrace;

use ::clap::Parser;

use ::futures::stream::SplitSink;

use ::futures_util::{SinkExt, StreamExt};

use ::http::response::Response;

use ::tokio::sync::{oneshot, RwLock};

use ::warp::{ws::WebSocket, Filter};

use ::std::{
    collections::BTreeMap,
    net::SocketAddr,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
};
use std::path::PathBuf;

use ::network::{ArrayDelta, Message};

#[derive(Parser, Debug)]
struct Args {
    /// Path to where the web files (i.e. HTML & JS) are stored.
    www_dir: PathBuf,
}

#[derive(Debug)]
pub struct BroadcastError {
    kind: BroadcastErrorKind,
    stack: Backtrace,
}

impl BroadcastError {
    fn new(kind: BroadcastErrorKind) -> Self {
        BroadcastError {
            kind,
            stack: Backtrace::new(),
        }
    }
}

impl From<::bincode::Error> for BroadcastError {
    fn from(error: ::bincode::Error) -> Self {
        BroadcastError::new(BroadcastErrorKind::Bincode(error))
    }
}

impl From<::warp::Error> for BroadcastError {
    fn from(error: ::warp::Error) -> Self {
        BroadcastError::new(BroadcastErrorKind::Warp(error))
    }
}

#[derive(Debug)]
pub enum BroadcastErrorKind {
    Bincode(::bincode::Error),
    Warp(::warp::Error),
}

#[derive(Debug)]
struct Services {
    next_id: AtomicUsize,
    listeners: BTreeMap<usize, SplitSink<WebSocket, ::warp::ws::Message>>,
}

impl Services {
    fn new() -> Services {
        Services {
            next_id: AtomicUsize::new(0),
            listeners: BTreeMap::new(),
        }
    }

    fn add_listener(&mut self, tx: SplitSink<WebSocket, ::warp::ws::Message>) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        self.listeners.insert(id, tx);
        id
    }

    fn remove_listener(&mut self, id: usize) -> Option<SplitSink<WebSocket, ::warp::ws::Message>> {
        self.listeners.remove(&id)
    }

    async fn set_rom(&mut self, setter_id: usize, rom: Vec<u8>) -> Result<(), BroadcastError> {
        let data: String = Message::Rom(rom).try_into()?;

        for (id, tx) in self.listeners.iter_mut() {
            if *id != setter_id {
                tx.send(::warp::ws::Message::text(&data)).await?;
            }
        }

        Ok(())
    }

    async fn set_bios(&mut self, setter_id: usize, bios: Vec<u8>) -> Result<(), BroadcastError> {
        let data: String = Message::Bios(bios).try_into()?;

        for (id, tx) in self.listeners.iter_mut() {
            if *id != setter_id {
                tx.send(::warp::ws::Message::text(&data)).await?;
            }
        }

        Ok(())
    }

    async fn broadcast_delta_snapshot(
        &mut self,
        sender_id: usize,
        snapshot: Vec<ArrayDelta>,
    ) -> Result<(), BroadcastError> {
        let data: String = Message::DeltaSnapshot(snapshot).try_into()?;

        for (id, tx) in self.listeners.iter_mut() {
            if *id != sender_id {
                tx.send(::warp::ws::Message::text(&data)).await?;
            }
        }

        Ok(())
    }

    async fn broadcast_snapshot(
        &mut self,
        sender_id: usize,
        snapshot: Vec<u8>,
    ) -> Result<(), BroadcastError> {
        let data: String = Message::Snapshot(snapshot).try_into()?;

        for (id, tx) in self.listeners.iter_mut() {
            if *id != sender_id {
                tx.send(::warp::ws::Message::text(&data)).await?;
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let is_running_flag = Arc::new(AtomicBool::new(true));
    let (warp_shutdown_tx, warp_shutdown_rx) = oneshot::channel::<()>();

    let services: Arc<RwLock<Services>> = Arc::new(RwLock::new(Services::new()));
    let services_filter = {
        let services = services.clone();
        warp::any().map(move || services.clone())
    };

    let warp_task = {
        let inline_paths = {
            // let bootstrap_css = warp::get()
            //     .and(warp::path("bootstrap.min.css"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "text/css; charset=utf-8")
            //             .body(server::html::BOOTSTRAP_CSS)
            //             .expect("Bootstrap.css was not bundled correctly")
            //     });
            // let bootstrap_css_map = warp::get()
            //     .and(warp::path("bootstrap.min.css.map"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "application/json; charset=utf-8")
            //             .body(server::html::BOOTSTRAP_CSS_MAP)
            //             .expect("Bootstrap.css.map was not bundled correctly")
            //     });

            // let bootstrap_js = warp::get()
            //     .and(warp::path("bootstrap.min.js"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "application/javascript; charset=utf-8")
            //             .body(server::html::BOOTSTRAP_JS)
            //             .expect("Bootstrap.js was not bundled correctly")
            //     });
            // let bootstrap_js_map = warp::get()
            //     .and(warp::path("bootstrap.min.js.map"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "application/json; charset=utf-8")
            //             .body(server::html::BOOTSTRAP_JS_MAP)
            //             .expect("Bootstrap.js.map was not bundled correctly")
            //     });

            // let handlebars_js = warp::get()
            //     .and(warp::path("handlebars.min.js"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "application/javascript; charset=utf-8")
            //             .body(server::html::HANDLEBARS_JS)
            //             .expect("Handlebars.js was not bundled correctly")
            //     });

            // let index_html = warp::get()
            //     .and(warp::path::end())
            //     .map(|| warp::reply::html(server::html::INDEX_HTML));

            // let wasm_ui_js = warp::get().and(warp::path("wasm_ui.js")).map(move || {
            //     Response::builder()
            //         .status(200)
            //         .header("content-type", "application/javascript; charset=utf-8")
            //         .body(server::html::WASM_UI_JS)
            //         .expect("Wasm_ui.js was not bundled correctly")
            // });
            // let wasm_ui_bg_js = warp::get().and(warp::path("wasm_ui_bg.js")).map(move || {
            //     Response::builder()
            //         .status(200)
            //         .header("content-type", "application/javascript; charset=utf-8")
            //         .body(server::html::WASM_UI_BG_JS)
            //         .expect("Wasm_ui.bg.js was not bundled correctly")
            // });

            // let wasm_ui_bg_wasm = warp::get()
            //     .and(warp::path("wasm_ui_bg.wasm"))
            //     .and(warp::path::end())
            //     .map(move || {
            //         Response::builder()
            //             .status(200)
            //             .header("content-type", "application/wasm")
            //             .body(server::html::WASM_UI_BG_WASM)
            //             .expect("UI WASM was not bundled correctly")
            //     });

            warp::get().and(warp::fs::dir(args.www_dir))
        };

        let websocket = warp::path("websocket")
            .and(warp::ws())
            .and(warp::addr::remote())
            .and(services_filter.clone())
            .map(
                |ws: warp::ws::Ws, remote: Option<SocketAddr>, services: Arc<RwLock<Services>>| {
                    ws.on_upgrade(move |socket| on_websocket(socket, remote, services))
                },
            );

        let routes = inline_paths.or(websocket);

        let (_addr, server) =
            warp::serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], 3030), async {
                warp_shutdown_rx.await.ok();
            });

        tokio::spawn(server)
    };

    log::info!("All tasks started.");
    /* http never sets the flag but all the other threads do -- So `await` on it first and then set the flag. */
    let _ = warp_task.await;
    is_running_flag.store(false, Ordering::SeqCst);

    log::error!("Server has stopped.");
}

async fn on_websocket(ws: WebSocket, remote: Option<SocketAddr>, services: Arc<RwLock<Services>>) {
    let (tx, mut rx) = ws.split();

    let id = services.write().await.add_listener(tx);

    loop {
        match rx.next().await {
            None => break,
            Some(Err(e)) => {
                log::warn!("Failure accepting message: {:?}", e);
                break;
            }
            Some(Ok(message)) => match message.to_str() {
                Err(_) => {
                    log::info!("Non-Text of length: {}", message.as_bytes().len());
                    break;
                }
                Ok(str) => match network::Message::try_from(str) {
                    Ok(Message::Bios(bios)) => {
                        log::info!("Bios -- {:?}", bios.len());

                        match services.write().await.set_bios(id, bios).await {
                            Err(e) => log::error!("Failed to send bios: {:?}", e),
                            _ => {}
                        }
                    }
                    Ok(Message::Rom(rom)) => {
                        log::info!("Rom -- {:?}", rom.len());

                        match services.write().await.set_rom(id, rom).await {
                            Err(e) => log::error!("Failed to send rom: {:?}", e),
                            _ => {}
                        }
                    }
                    Ok(Message::DeltaSnapshot(snapshot)) => {
                        log::info!("Snapshot (Delta) -- {:?}", snapshot.len());

                        match services
                            .write()
                            .await
                            .broadcast_delta_snapshot(id, snapshot)
                            .await
                        {
                            Err(e) => log::error!("Failed to send delta: {:?}", e),
                            _ => {}
                        }
                    }
                    Ok(Message::Snapshot(snapshot)) => {
                        log::info!("Snapshot -- {:?}", snapshot.len());

                        match services
                            .write()
                            .await
                            .broadcast_snapshot(id, snapshot)
                            .await
                        {
                            Err(e) => log::error!("Failed to send snapshot: {:?}", e),
                            _ => {}
                        }
                    }
                    Err(e) => {
                        log::warn!("Fail to parse message. {:?}", e);
                        break;
                    }
                },
            },
        }
    }

    if let Some(tx) = services.write().await.remove_listener(id) {
        match rx.reunite(tx).unwrap().close().await {
            Err(e) => log::debug!("Failed to close ws: {}", e),
            _ => {}
        }
    }
}
