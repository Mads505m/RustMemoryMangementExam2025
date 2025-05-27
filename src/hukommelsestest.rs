use windows::{
    Win32::System::ProcessStatus::{K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS},
    Win32::System::Threading::GetCurrentProcess,
};

#[derive(Debug)]
struct MyObject {
    a: i32,
    b: i32,
    data: [u8; 32],
}

fn get_memory_usage_kb() -> u64 {
    unsafe {
        let handle = GetCurrentProcess();
        let mut mem_counters = PROCESS_MEMORY_COUNTERS::default();
        if K32GetProcessMemoryInfo(
            handle,
            &mut mem_counters,
            std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
        )
            .as_bool()
        {
            mem_counters.WorkingSetSize as u64 / 1024
        } else {
            0
        }
    }
}

fn benchmark(count: usize) {
    let mut memory_log = Vec::new();
    memory_log.push(get_memory_usage_kb());

    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        objects.push(MyObject {
            a: 42,
            b: 7,
            data: [0u8; 32],
        });
        memory_log.push(get_memory_usage_kb());
    }

    memory_log.push(get_memory_usage_kb());

    let min = memory_log.iter().min().unwrap();
    let max = memory_log.iter().max().unwrap();
    let avg = memory_log.iter().sum::<u64>() / memory_log.len() as u64;

    println!("Antal objekter: {}", count);
    println!("Min: {} KB", min);
    println!("Gennemsnit: {} KB", avg);
    println!("Peak: {} KB", max);
}

fn main() {
    for &count in &[10, 100, 1_000, 10_000, 100_000, 10_000_000] {
        benchmark(count);
    }
}
