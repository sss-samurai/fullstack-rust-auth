use tokio_postgres::{Client, NoTls, Socket};
use tokio_postgres::tls::NoTlsStream;
use tokio::sync::{Mutex, Semaphore};
use std::collections::VecDeque;
use std::sync::Arc;

pub struct MyConnection {
    pub client: Client,
}

impl MyConnection {
pub async fn new(
    conn_str: &str,
) -> Result<
    (
        Self,
        tokio_postgres::Connection<Socket, NoTlsStream>
    ),
    tokio_postgres::Error,
> {
    let (client, connection) = tokio_postgres::connect(conn_str, NoTls).await?;
    Ok((MyConnection { client }, connection))
}


    
}

pub struct AsyncConnectionPool {
    connections: Arc<Mutex<VecDeque<MyConnection>>>,
    semaphore: Arc<Semaphore>,
    conn_str: String,
}

impl AsyncConnectionPool {
    pub async fn new(conn_str: &str, max_size: usize) -> Self {
        let mut connections = VecDeque::new();
        for _ in 0..max_size {
            let (conn, connection_task) = MyConnection::new(conn_str).await.unwrap();
            tokio::spawn(async move {
                if let Err(e) = connection_task.await {
                    eprintln!("connection error: {}", e);
                }
            });
            connections.push_back(conn);
        }

        AsyncConnectionPool {
            connections: Arc::new(Mutex::new(connections)),
            semaphore: Arc::new(Semaphore::new(max_size)),
            conn_str: conn_str.to_string(),
        }
    }

    pub async fn get(&self) -> Option<MyConnection> {
        let permit = self.semaphore.clone().acquire_owned().await.ok()?;
        let mut pool = self.connections.lock().await;

        pool.pop_front().or_else(|| {
            drop(permit);
            None
        })
    }

    pub async fn return_connection(&self, conn: MyConnection) {
        let mut pool = self.connections.lock().await;
        pool.push_back(conn);
        self.semaphore.add_permits(1);
    }
}
