use std::{
    ffi::OsString,
    io,
    process::{Command, Output},
};

use tokio::sync::mpsc;

/// Can be used to run a command in a separate thread.
/// What's wrong with it?
pub struct Worker {
    args_tx: mpsc::Sender<String>,
    out_rx: mpsc::Receiver<io::Result<Output>>,
}

impl Worker {
    pub fn new(command: OsString) -> Self {
        let (args_tx, args_rx) = mpsc::channel(12);
        let (out_tx, out_rx) = mpsc::channel(12);

        let task = WorkerTask {
            command,
            args_rx,
            out_tx,
        };

        tokio::spawn(task.run());

        Self { args_tx, out_rx }
    }

    pub async fn run(&mut self, arguments: String) -> io::Result<Output> {
        self.args_tx
            .send(arguments)
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Worker task is dead"))?;

        self.out_rx
            .recv()
            .await
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Worker task is dead"))?
    }
}

struct WorkerTask {
    command: OsString,
    args_rx: mpsc::Receiver<String>,
    out_tx: mpsc::Sender<io::Result<Output>>,
}

impl WorkerTask {
    async fn run(mut self) {
        while let Some(args) = self.args_rx.recv().await {
            let command = self.command.clone();
            let out_tx = self.out_tx.clone();

            tokio::task::spawn_blocking(move || {
                let mut command = Command::new(command);
                command.args(args.split_whitespace());

                let result = command.output();
                let _ = out_tx.send(result);
            });
        }
    }
}
