//! Processes independent items on all available cores with the standard
//! library's scoped threads.

use std::panic;
use std::sync::Mutex;
use std::thread;

/// How many items a thread takes from the queue at a time.
const BATCH: usize = 16;

/// Applies `f` to each item and returns the results in the order of `items`.
///
/// The threads take items from a shared queue, [`BATCH`] at a time, so the
/// work stays balanced when some items take much longer than others. If the
/// number of cores is unknown, as on most Wasm targets, `map` processes the
/// items on the current thread.
pub fn map<T: Send, R: Send>(items: Vec<T>, f: impl Fn(T) -> R + Sync) -> Vec<R> {
    let threads = thread::available_parallelism()
        .map_or(1, |n| n.get())
        .min(items.len().div_ceil(BATCH));
    if threads <= 1 {
        return items.into_iter().map(f).collect();
    }
    let queue = Mutex::new(items.into_iter().enumerate());
    let mut results = thread::scope(|scope| {
        let workers = (0..threads)
            .map(|_| {
                scope.spawn(|| {
                    let mut results = Vec::new();
                    loop {
                        let batch = queue
                            .lock()
                            .unwrap()
                            .by_ref()
                            .take(BATCH)
                            .collect::<Vec<_>>();
                        if batch.is_empty() {
                            return results;
                        }
                        results.extend(batch.into_iter().map(|(i, item)| (i, f(item))));
                    }
                })
            })
            .collect::<Vec<_>>();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap_or_else(|e| panic::resume_unwind(e)))
            .collect::<Vec<_>>()
    });
    results.sort_unstable_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, result)| result).collect()
}

/// Calls `f` on each item, on all available cores. See [`map`].
pub fn for_each<T: Send>(items: Vec<T>, f: impl Fn(T) + Sync) {
    map(items, f);
}

#[cfg(test)]
mod tests {
    #[test]
    fn map_keeps_order() {
        let items = (0..1000).collect::<Vec<u32>>();
        let squares = super::map(items.clone(), |i| i * i);
        assert_eq!(squares, items.iter().map(|i| i * i).collect::<Vec<_>>());
    }

    #[test]
    fn map_empty() {
        assert!(super::map(Vec::<u32>::new(), |i| i).is_empty());
    }
}
