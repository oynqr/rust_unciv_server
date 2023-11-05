pub enum Listener {
    TcpListener(async_net::TcpListener),
    #[cfg(unix)]
    UnixListener(async_net::unix::UnixListener),
}

pub fn get_prebound_listener() -> Option<Listener> {
    let mut listenfd = listenfd::ListenFd::from_env();

    #[cfg(unix)]
    if let Some(listener) = listenfd
        .take_unix_listener(0)
        .ok()
        .flatten()
        .and_then(|std_listener| {
            async_net::unix::UnixListener::try_from(std_listener).ok()
        })
    {
        return Some(Listener::UnixListener(listener));
    }

    listenfd
        .take_tcp_listener(0)
        .ok()
        .flatten()
        .and_then(|std_listener| {
            async_net::TcpListener::try_from(std_listener)
                .ok()
                .and_then(|listener| Some(Listener::TcpListener(listener)))
        })
}
