use std::path::PathBuf;

pub fn init(dir: PathBuf) {
    let _ = std::fs::create_dir_all(&dir);
    let appender = tracing_appender::rolling::daily(dir, "rikavpn.log");
    let (writer, guard) = tracing_appender::non_blocking(appender);
    Box::leak(Box::new(guard));
    let _ = tracing_subscriber::fmt()
        .with_writer(writer)
        .with_ansi(false)
        .try_init();
}
