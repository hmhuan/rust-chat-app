
use tokio::io::AsyncBufReadExt;
use tokio::io::BufReader;
use tokio::{
    net::TcpListener, 
    io::{AsyncWriteExt},
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let tx = tx.clone(); 
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();
    
            loop {
                tokio::select! {
                    bytes_read = reader.read_line(&mut line) => {
                        if bytes_read.unwrap() == 0 {
                            break;
                        }
                        tx.send(line.clone()).unwrap();
                        line.clear();
                    },
                    msg = rx.recv() => {
                        writer.write_all(msg.unwrap().as_bytes()).await.unwrap();     
                    }
                }                
            }
        });

    }
}
