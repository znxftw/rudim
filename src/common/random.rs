#[cfg(test)]
use std::sync::Mutex;
use std::sync::atomic::{AtomicI32, AtomicU64, Ordering};

static ULONG_STATE: AtomicU64 = AtomicU64::new(1804289383);
static INT_STATE: AtomicI32 = AtomicI32::new(1804289383);

// Ensure tests don't interleave seed resets
#[cfg(test)]
static TEST_MUTEX: Mutex<()> = Mutex::new(());

// TODO: can be moved into build.rs
pub fn next_u64() -> u64 {
    let mut current = ULONG_STATE.load(Ordering::Relaxed);
    loop {
        let mut next = current;
        next ^= next << 13;
        next ^= next >> 7;
        next ^= next << 17;
        match ULONG_STATE.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed)
        {
            Ok(_) => return next,
            Err(v) => current = v,
        }
    }
}

pub fn next_i32() -> i32 {
    let mut current = INT_STATE.load(Ordering::Relaxed);
    loop {
        let mut next = current;
        next ^= next << 13;
        next ^= next >> 17;
        next ^= next << 5;
        match INT_STATE.compare_exchange_weak(current, next, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return next,
            Err(v) => current = v,
        }
    }
}

pub fn reset_seed() {
    ULONG_STATE.store(1804289383, Ordering::Relaxed);
    INT_STATE.store(1804289383, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_unique_u64s() {
        let _guard = TEST_MUTEX.lock().unwrap();
        reset_seed();
        let mut generated = HashSet::new();
        for _ in 0..500 {
            let num = next_u64();
            assert!(
                generated.insert(num),
                "Collision detected for ulong number: {}",
                num
            );
        }
    }

    #[test]
    fn test_unique_i32s() {
        let _guard = TEST_MUTEX.lock().unwrap();
        reset_seed();
        let mut generated = HashSet::new();
        for _ in 0..500 {
            let num = next_i32();
            assert!(
                generated.insert(num),
                "Collision detected for int number: {}",
                num
            );
        }
    }
}
