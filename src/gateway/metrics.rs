use std::sync::atomic::{AtomicU64, Ordering};

use std::time::Instant;

#[derive(Debug)]
pub struct ShardMetrics {
    events_total: AtomicU64,

    reconnects_total: AtomicU64,

    heartbeats_total: AtomicU64,

    send_delays_total: AtomicU64,

    started: std::sync::Mutex<Option<Instant>>,
}

impl Default for ShardMetrics {
    fn default() -> Self {
        Self {
            events_total: AtomicU64::new(0),

            reconnects_total: AtomicU64::new(0),

            heartbeats_total: AtomicU64::new(0),

            send_delays_total: AtomicU64::new(0),

            started: std::sync::Mutex::new(Some(Instant::now())),
        }
    }
}

impl ShardMetrics {
    pub fn record_event(&self) {
        self.events_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_reconnect(&self) {
        self.reconnects_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_heartbeat(&self) {
        self.heartbeats_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_send_delay(&self) {
        self.send_delays_total.fetch_add(1, Ordering::Relaxed);
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn events_per_second(&self) -> f64 {
        let elapsed = self
            .started
            .lock()
            .ok()
            .and_then(|g| *g)
            .map_or(1.0, |s| s.elapsed().as_secs_f64())
            .max(0.001);

        self.events_total.load(Ordering::Relaxed) as f64 / elapsed
    }

    pub fn render_prometheus(&self, shard: u32) -> String {
        format!(
            "rustydiscord_shard_events_total{{shard=\"{shard}\"}} {}\nrustydiscord_shard_events_per_second{{shard=\"{shard}\"}} {:.2}\nrustydiscord_shard_reconnects_total{{shard=\"{shard}\"}} {}\nrustydiscord_shard_heartbeats_total{{shard=\"{shard}\"}} {}\nrustydiscord_shard_send_delays_total{{shard=\"{shard}\"}} {}\n",
            self.events_total.load(Ordering::Relaxed),
            self.events_per_second(),
            self.reconnects_total.load(Ordering::Relaxed),
            self.heartbeats_total.load(Ordering::Relaxed),
            self.send_delays_total.load(Ordering::Relaxed),
        )
    }
}
