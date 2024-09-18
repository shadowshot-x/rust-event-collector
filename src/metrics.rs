

pub async fn handle_receiver(rx: &mut tokio::sync::mpsc::Receiver<String>){
    while let Some(metricinfo) = rx.recv().await {
        // write to some metrics backend.
        println!("Received : {}", metricinfo)
    }
}