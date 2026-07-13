use amun_bench::time_op;
use std::io::Write;

#[test]
fn n163_bench_wal_write_and_read() {
    let path = "/tmp/amun-bench-wal.log";
    let _ = std::fs::remove_file(path);

    let write_result = time_op("wal_write_1000_entries", || {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        for i in 0..1000u64 {
            let entry = format!("WAL_ENTRY_{}_DATA_{}\n", i, "A".repeat(100));
            file.write_all(entry.as_bytes()).unwrap();
        }
        file.sync_all().unwrap();
    });

    let read_result = time_op("wal_read_1000_entries", || {
        let content = std::fs::read_to_string(path).unwrap();
        let lines = content.lines().count();
        assert_eq!(lines, 1000);
    });

    println!(
        "WAL write: {}ms, read: {}ms",
        write_result.duration_ms, read_result.duration_ms
    );
    assert!(write_result.duration_ms < 500, "WAL write too slow");
    assert!(read_result.duration_ms < 200, "WAL read too slow");

    let _ = std::fs::remove_file(path);
}
