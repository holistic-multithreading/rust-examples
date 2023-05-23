use std::thread::JoinHandle;

#[allow(unused)]
pub fn count_sum_single_thread(num: usize) -> usize {
    let mut total: usize = 0;
    for i in 0..=num {
        total += i % 10;
    }
    total
}

#[allow(unused)]
pub fn count_sum_multithreaded(range: usize) -> usize {
    let cpu_count = num_cpus::get();
    let chunk_size = range / cpu_count;

    let mut thread_handles: Vec<JoinHandle<usize>> = Vec::with_capacity(cpu_count);

    for i in 0..num_cpus::get() {
        let thread_handle = std::thread::spawn(move || {
            let start = chunk_size * i + 1;
            let mut end = start + chunk_size;
            if i + 1 == cpu_count {
                let remaining = range - (chunk_size * cpu_count);
                end += remaining;
            }
            let mut sum = 0;
            for i in start..end {
                sum += i % 10;
            }
            sum
        });
        thread_handles.push(thread_handle);
    }

    thread_handles
        .into_iter()
        .fold(0, |sum, cur | sum + cur.join().unwrap())
}

#[test]
fn test_sum_thousand_single_thread() {
    let res = count_sum_single_thread(1_000);
    assert_eq!(res, 4500);

}

#[test]
fn test_sum_million_single_thread() {
    let res = count_sum_single_thread(1_000_000);
    assert_eq!(res, 4500000);

}

#[test]
fn test_sum_billion_single_thread() {
    let res = count_sum_single_thread(1_000_000_000);
    assert_eq!(res, 4500000000);

}


#[test]
fn test_sum_thousand_multithreaded() {
    let res = count_sum_multithreaded(1_000);
    assert_eq!(res, 4500);

}

#[test]
fn test_sum_million_multithreaded() {
    let res = count_sum_multithreaded(1_000_000);
    assert_eq!(res, 4500000);
}

#[test]
fn test_sum_billion_multithreaded() {
    let start = std::time::Instant::now();
    let res = count_sum_multithreaded(1_000_000_000);
    let finish = std::time::Instant::now();
    let duration = finish.duration_since(start);
    println!("it took millis: {}", duration.as_millis());
    assert_eq!(res, 4500000000);
}

