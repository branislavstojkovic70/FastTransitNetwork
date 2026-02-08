use rayon::ThreadPool;
use std::sync::OnceLock;

#[allow(dead_code)]
static THREAD_POOL: OnceLock<ThreadPool> = OnceLock::new();

#[allow(dead_code)]
pub fn get_thread_pool(num_threads: usize) -> &'static ThreadPool {
    THREAD_POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .unwrap()
    })
}