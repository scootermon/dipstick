use std::sync::Arc;
use std::time::{Duration, SystemTime};

use dipstick_proto::xcp::v1::{
    CtoCommand, CtoCommandContent, CtoResponse, CtoResponseContent, PeriodicShortUploadResponse,
    ShortUploadCommand,
};
use futures::stream::BoxStream;
use futures::StreamExt;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::time::MissedTickBehavior;
use tokio_stream::wrappers::BroadcastStream;
use tonic::Result;

use super::session::Session;

pub type Stream = BoxStream<'static, Result<PeriodicShortUploadResponse>>;

pub struct Worker {
    created_at: SystemTime,
    task: Option<JoinHandle<()>>,
    interval: Duration,
    tx: broadcast::Sender<PeriodicShortUploadResponse>,
}

impl Worker {
    pub fn new(
        session: Arc<Session>,
        command: ShortUploadCommand,
        interval: Duration,
    ) -> (Self, Stream) {
        let (tx, rx) = broadcast::channel(16); // TODO
        let task = tokio::spawn(run(session, command, interval, tx.clone()));
        let this = Self {
            created_at: SystemTime::now(),
            task: Some(task),
            interval,
            tx,
        };
        (this, new_stream(rx))
    }

    pub fn interval(&self) -> Duration {
        self.interval
    }

    pub fn is_running(&self) -> bool {
        self.tx.receiver_count() > 0 && self.task.as_ref().is_some_and(|task| !task.is_finished())
    }

    pub fn subscribe(&self) -> Option<Stream> {
        if !self.is_running() {
            return None;
        }
        let rx = self.tx.subscribe();
        Some(new_stream(rx))
    }

    pub async fn shutdown(&mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let _ = task.await;
        }
    }
}

fn new_stream(rx: broadcast::Receiver<PeriodicShortUploadResponse>) -> Stream {
    BroadcastStream::new(rx)
        .filter_map(|res| std::future::ready(res.ok().map(Ok)))
        .boxed()
}

async fn run(
    session: Arc<Session>,
    command: ShortUploadCommand,
    interval: Duration,
    tx: broadcast::Sender<PeriodicShortUploadResponse>,
) {
    let mut interval = tokio::time::interval(interval);
    interval.set_missed_tick_behavior(MissedTickBehavior::Skip);

    let command = CtoCommand {
        cto_command_content: Some(CtoCommandContent::ShortUpload(command)),
        ..Default::default()
    };

    while tx.receiver_count() > 0 {
        interval.tick().await;
        match session.raw_command(command.clone()).await {
            Ok(CtoResponse {
                timestamp,
                cto_response_content,
            }) => {
                let mut response = PeriodicShortUploadResponse {
                    timestamp,
                    ..Default::default()
                };
                match cto_response_content {
                    Some(CtoResponseContent::ShortUpload(content)) => {
                        response.data = content.data;
                    }
                    Some(CtoResponseContent::Negative(content)) => {
                        response.error = content.code;
                    }
                    _ => unreachable!(),
                }
                let _ = tx.send(response);
            }
            Err(_err) => {
                // TODO
                break;
            }
        }
    }
}
