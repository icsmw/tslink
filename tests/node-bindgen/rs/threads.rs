use futures::{
    channel::{mpsc, oneshot},
    executor, StreamExt,
};
use node_bindgen::derive::node_bindgen;
use std::thread::spawn;
use tslink::tslink;

#[derive(Debug, Clone)]
enum Command {
    IncValue(i32),
    Shutdown,
}

#[derive(Debug, Clone)]
struct StructThreads {
    tx: Option<mpsc::UnboundedSender<Command>>,
}

#[tslink(class)]
#[node_bindgen]
impl StructThreads {
    #[tslink(constructor)]
    #[node_bindgen(constructor)]
    pub fn new() -> Self {
        Self { tx: None }
    }

    #[tslink(snake_case_naming, exception_suppression)]
    #[node_bindgen(mt)]
    async fn rt<F: Fn(i32) + Send + 'static>(&mut self, cb: F) -> Result<(), String> {
        let (tx, mut rx) = mpsc::unbounded::<Command>();
        let (confirm_tx, confirm_rx) = oneshot::channel::<()>();
        self.tx = Some(tx);
        spawn(move || {
            executor::block_on(async {
                let _ = confirm_tx.send(());
                while let Some(cmd) = rx.next().await {
                    match cmd {
                        Command::IncValue(v) => {
                            cb(v + 1);
                        }
                        Command::Shutdown => {
                            cb(-1);
                            break;
                        }
                    }
                }
            });
        });
        confirm_rx
            .await
            .map_err(|_| "Fail to confirm thread creating".to_string())
    }

    #[tslink(snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn inc_value(&self, v: i32) -> Result<(), String> {
        if let Some(tx) = self.tx.as_ref() {
            tx.unbounded_send(Command::IncValue(v))
                .map_err(|e| e.to_string())
        } else {
            Err("Channel isn't up".to_string())
        }
    }

    #[tslink(snake_case_naming, exception_suppression)]
    #[node_bindgen]
    fn shutdown(&self) -> Result<(), String> {
        if let Some(tx) = self.tx.as_ref() {
            tx.unbounded_send(Command::Shutdown)
                .map_err(|e| e.to_string())
        } else {
            Err("Channel isn't up".to_string())
        }
    }
}
