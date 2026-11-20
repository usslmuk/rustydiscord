use core::time::Duration;
use std::collections::VecDeque;

use tokio::time::Instant;

pub(crate) const WINDOW: Duration = Duration::from_secs(61);

pub(crate) const COMMANDS_PER_WINDOW: usize = 115;

#[derive(Debug, Default)]
pub(crate) struct SendLimit {
    sent: VecDeque<Instant>,
}

impl SendLimit {
    pub(crate) fn try_acquire(&mut self, now: Instant) -> Result<(), Instant> {
        while self.sent.front().is_some_and(|&at| at + WINDOW <= now) {
            self.sent.pop_front();
        }
        match self.sent.front() {
            Some(&oldest) if self.sent.len() >= COMMANDS_PER_WINDOW => Err(oldest + WINDOW),
            _ => {
                self.sent.push_back(now);
                Ok(())
            }
        }
    }

    pub(crate) fn clear(&mut self) {
        self.sent.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::{COMMANDS_PER_WINDOW, Duration, Instant, SendLimit, WINDOW};

    const MS: Duration = Duration::from_millis(1);

    #[test]
    fn allows_a_full_window_then_waits_for_the_oldest_send() {
        let start = Instant::now();
        let mut limit = SendLimit::default();
        for i in 0..COMMANDS_PER_WINDOW {
            let at = start + MS * u32::try_from(i).unwrap();
            assert_eq!(limit.try_acquire(at), Ok(()), "command {i}");
        }
        let now = start + Duration::from_secs(1);
        assert_eq!(limit.try_acquire(now), Err(start + WINDOW));

        assert_eq!(limit.sent.len(), COMMANDS_PER_WINDOW);
    }

    #[test]
    fn slots_free_up_one_by_one() {
        let start = Instant::now();
        let mut limit = SendLimit::default();
        for i in 0..COMMANDS_PER_WINDOW {
            let at = start + MS * u32::try_from(i).unwrap();
            limit.try_acquire(at).unwrap();
        }

        assert_eq!(limit.try_acquire(start + WINDOW), Ok(()));
        assert_eq!(limit.try_acquire(start + WINDOW), Err(start + MS + WINDOW));
        assert_eq!(limit.try_acquire(start + MS + WINDOW), Ok(()));
    }

    #[test]
    fn never_exceeds_the_limit_in_any_window() {
        let start = Instant::now();
        let mut limit = SendLimit::default();
        let mut sent = Vec::new();
        let mut now = start;

        while now < start + WINDOW * 5 {
            match limit.try_acquire(now) {
                Ok(()) => sent.push(now),
                Err(at) => now = at,
            }
        }
        for (i, &first) in sent.iter().enumerate() {
            let in_window = sent[i..].iter().take_while(|&&at| at < first + WINDOW);
            assert!(in_window.count() <= COMMANDS_PER_WINDOW);
        }
        assert_eq!(sent.len(), COMMANDS_PER_WINDOW * 5);
    }

    #[test]
    fn clear_starts_a_fresh_window() {
        let now = Instant::now();
        let mut limit = SendLimit::default();
        for _ in 0..COMMANDS_PER_WINDOW {
            limit.try_acquire(now).unwrap();
        }
        assert!(limit.try_acquire(now).is_err());
        limit.clear();
        assert_eq!(limit.try_acquire(now), Ok(()));
    }
}
