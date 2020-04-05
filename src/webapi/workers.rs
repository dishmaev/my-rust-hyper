use super::{connectors, errors};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug)]
pub enum SignalCode {
    Exit = 0,
    JobNo1 = 1
}

pub async fn event_publisher_worker(
    cancel_flag: Arc<AtomicBool>,
    mut receiver: mpsc::Receiver<SignalCode>,
) -> connectors::Result<()> {
    const TASK: &str = "event publisher";
    debug!("start {}", TASK);
    loop {
        match receiver.recv().await {
            Some(m) => {
                debug!("{} receive signal {:?}", TASK, m);
                match m {
                    SignalCode::Exit => {
                        return Ok({});
                    }
                    _ => {}
                };
            },
            None => {
                if cancel_flag.load(Ordering::SeqCst) {
                    debug!("{} cancel flag", TASK);
                    return Ok({});
                } else {
                    return Err(errors::ChannelError.into());
                }
            }
        };
    }
}

pub async fn command_executor_worker(
    cancel_flag: Arc<AtomicBool>,
    mut receiver: mpsc::Receiver<SignalCode>,
) -> connectors::Result<()> {
    const TASK: &str = "command executor";
    debug!("start {}", TASK);
    loop {
        match receiver.recv().await {
            Some(m) => {
                debug!("{} receive signal {:?}", TASK, m);
                match m {
                    SignalCode::Exit => {
                        return Ok({});
                    }
                    _ => {}
                };
            },
            None => {
                if cancel_flag.load(Ordering::SeqCst) {
                    debug!("{} cancel flag", TASK);
                    return Ok({});
                } else {
                    return Err(errors::ChannelError.into());
                }
            }
        };
    }
}
