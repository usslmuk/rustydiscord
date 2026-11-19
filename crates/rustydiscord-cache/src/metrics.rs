use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct CacheMetrics {
    hits: AtomicU64,

    misses: AtomicU64,

    inserts: AtomicU64,

    evictions: AtomicU64,
}

impl CacheMetrics {
    pub fn record_hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_miss(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_insert(&self) {
        self.inserts.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_eviction(&self) {
        self.evictions.fetch_add(1, Ordering::Relaxed);
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn hit_rate(&self) -> f64 {
        let h = self.hits.load(Ordering::Relaxed) as f64;

        let m = self.misses.load(Ordering::Relaxed) as f64;

        if h + m == 0.0 { 0.0 } else { h / (h + m) }
    }

    pub fn render_prometheus(&self) -> String {
        format!(
            "rustydiscord_cache_hits_total {}\nrustydiscord_cache_misses_total {}\nrustydiscord_cache_inserts_total {}\nrustydiscord_cache_evictions_total {}\nrustydiscord_cache_hit_ratio {}\n",
            self.hits.load(Ordering::Relaxed),
            self.misses.load(Ordering::Relaxed),
            self.inserts.load(Ordering::Relaxed),
            self.evictions.load(Ordering::Relaxed),
            self.hit_rate(),
        )
    }
}
