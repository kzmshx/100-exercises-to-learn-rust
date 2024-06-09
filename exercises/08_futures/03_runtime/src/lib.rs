// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply clients by sending
//  the `Display` representation of the `reply` argument as a response.
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use tokio::io::AsyncWriteExt;
use tokio::join;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // `T` cannot be cloned. How do you share it between the two server tasks?
    T: Display + Send + Sync + 'static,
{
    let reply = Arc::new(RwLock::new(reply));
    let (r1, r2) = join!(
        tokio::spawn(_fixed_reply(first, reply.clone())),
        tokio::spawn(_fixed_reply(second, reply.clone()))
    );
    r1.unwrap();
    r2.unwrap();
}

async fn _fixed_reply<T: Display + Send + Sync + 'static>(
    listener: TcpListener,
    reply: Arc<RwLock<T>>,
) {
    loop {
        let (mut s, _) = listener.accept().await.unwrap();
        let (_, mut w) = s.split();
        let reply = format!("{}", reply.read().unwrap());
        w.write_all(reply.as_bytes()).await.unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
