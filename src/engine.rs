use std::sync::atomic::{AtomicBool, Ordering};

static IS_READY: AtomicBool = AtomicBool::new(false);

pub fn reset() {
    IS_READY.store(false, Ordering::Relaxed);
}

pub fn set_ready() {
    IS_READY.store(true, Ordering::Relaxed);
}

pub fn is_ready() -> bool {
    IS_READY.load(Ordering::Relaxed)
}
