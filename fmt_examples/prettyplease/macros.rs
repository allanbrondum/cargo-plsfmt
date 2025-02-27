fn test() {
    println!("{}", "test");
    trace!(test = % "test", test = % "test", "test");
}
async fn run_impl<Inc: MessageDeserialize, Outg: MessageSerialize>(
    backend: &mut WsBackend<Inc, Outg>,
) -> ActorResult<()> {
    let mut ping_interval = tokio::time::interval(Duration::from_secs(30));
    let mut last_rx_activity = Instant::now();
    loop {
        select! {
            frontend_msg = backend.frontend_handle.recv() => { backend
            .process_frontend_msg(frontend_msg ?). await
            .context("process frontend message") ?; } ws_msg_option = backend.ws.next()
            => { let ws_msg = ws_msg_option.context("websocket stream ended") ?
            .context("websocket error") ?; last_rx_activity = Instant::now(); backend
            .process_ws_msg(ws_msg). await .context("process websocket msg") ?; } _ =
            ping_interval.tick() => { backend.send_ping(). await ?; } _ =
            tokio::time::sleep_until(last_rx_activity + Duration::from_secs(60)) => {
            return Err(anyhow!("websocket presumed dead due to lack of activity")
            .into()); }
        }
    }
}
async fn run(mut backend: RpcBackend) -> ActorResult<()> {
    loop {
        select! {
            frontend_msg = backend.frontend_handle.recv() => { backend
            .process_frontend_msg(frontend_msg ?); } _ = backend.client.ws_client
            .on_disconnect() => { return Err(anyhow!("pyth agent client disconnected")
            .into()); }
        }
    }
}
