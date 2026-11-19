use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Default)]
pub struct RestMetrics {
    requests_total: AtomicU64,

    responses_2xx: AtomicU64,

    responses_429: AtomicU64,

    responses_5xx: AtomicU64,

    retries_total: AtomicU64,
}

impl RestMetrics {
    pub fn record_request(&self) {
        self.requests_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_status(&self, status: u16) {
        if (200..300).contains(&status) {
            self.responses_2xx.fetch_add(1, Ordering::Relaxed);
        } else if status == 429 {
            self.responses_429.fetch_add(1, Ordering::Relaxed);
        } else if (500..600).contains(&status) {
            self.responses_5xx.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn record_retry(&self) {
        self.retries_total.fetch_add(1, Ordering::Relaxed);
    }

    pub fn render_prometheus(&self) -> String {
        format!(
            "rustydiscord_rest_requests_total {}\nrustydiscord_rest_2xx_total {}\nrustydiscord_rest_429_total {}\nrustydiscord_rest_5xx_total {}\nrustydiscord_rest_retries_total {}\n",
            self.requests_total.load(Ordering::Relaxed),
            self.responses_2xx.load(Ordering::Relaxed),
            self.responses_429.load(Ordering::Relaxed),
            self.responses_5xx.load(Ordering::Relaxed),
            self.retries_total.load(Ordering::Relaxed),
        )
    }
}
