use tracing_subscriber::{
    EnvFilter, fmt::time::FormatTime, layer::SubscriberExt, util::SubscriberInitExt,
};

struct ChronoLocalTimer;

impl FormatTime for ChronoLocalTimer {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        write!(
            w,
            "{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")
        )
    }
}

pub fn swan_tracing() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "swan=debug".into()))
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(false)
                .with_timer(ChronoLocalTimer)
                .with_file(false)
                .compact()
                .pretty(),
        )
        .init();
}

#[macro_export]
macro_rules! log_error {
    ($request_id:expr, $($arg:tt)*) => {{
        let short_id = &$request_id.as_str()[..8];
        tracing::error!("[{}] {}", short_id, format!($($arg)*))
    }};
}
