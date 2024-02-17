// Most async runtimes only allow IO tasks to run concurrently. This means that CPU blocking tasks
// will block the executor and prevent other tasks from being executed. An easy workaround is to
// use async equivalent methods where possible.
//
//
use futures::future::join_all;
use std::time::Instant;

async fn sleep_ms_consecutive(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    println!("future {id} slept for {duration_ms}ms, finished after {}ms", start.elapsed().as_millis());
}

async fn sleep_ms(start: &tokio::time::Instant, id: u64, duration_ms: u64) {
    let _ = tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
    println!("future {id} slept for {duration_ms}ms, finished after {}ms", start.elapsed().as_millis());
}
#[tokio::main(flavor = "current_thread")] // current_thread puts all the tasks on a single thread
// to make the effect more obvious
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms_consecutive(&start, t, t * 10));
    join_all(sleep_futures).await;
    
    let start2 = tokio::time::Instant::now();
    let sleep_futures2 = (1..=10).map(|t| sleep_ms(&start2, t, t * 10));
    join_all(sleep_futures2).await;
}

// sleep_ms_consecutive has sleeps that run consecutively rather than concurrently
//
// sleep_ms has `tokio::time::sleep` instead
//
// Another fix would be to use `tokio::task::spawn_blocking` which spawns an actual thread and
// transforms its handle into a future without blocking the executor
//
// You should not think of tasks as OS threads. They do not map 1 to 1 and most executors will
// allow many tasks to run on a single OS thread. This is particularly problematic when interacting
// with other libraries via FFI, where that library might depend on thread-local storage or map to
// specific OS threads (e.g., CUDA). Prefer `tokio::task::spawn_blocking` in such situations
//
// Use sync mutexes with care. Holding a mutex over an `.await` may cause another task to block,
// and that task may be running on the same thread.
