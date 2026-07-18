use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct PerfTimer {
    label: &'static str,
    start: std::time::Instant,
}

impl PerfTimer {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            start: std::time::Instant::now(),
        }
    }
}

impl Drop for PerfTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed().as_micros() as u64;
        match self.label {
            "process_vote" => {
                PROCESS_VOTE_COUNT.fetch_add(1, Ordering::Relaxed);
                PROCESS_VOTE_TOTAL.fetch_add(elapsed, Ordering::Relaxed);
                track_max(&PROCESS_VOTE_MAX, elapsed);
                if elapsed > 1000 {
                    PROCESS_VOTE_SLOW.fetch_add(1, Ordering::Relaxed);
                }
            }
            "try_advance" => {
                TRY_ADVANCE_COUNT.fetch_add(1, Ordering::Relaxed);
                TRY_ADVANCE_TOTAL.fetch_add(elapsed, Ordering::Relaxed);
                track_max(&TRY_ADVANCE_MAX, elapsed);
                if elapsed > 1000 {
                    TRY_ADVANCE_SLOW.fetch_add(1, Ordering::Relaxed);
                }
            }
            "verify_sig" => {
                VERIFY_SIG_COUNT.fetch_add(1, Ordering::Relaxed);
                VERIFY_SIG_TOTAL.fetch_add(elapsed, Ordering::Relaxed);
                track_max(&VERIFY_SIG_MAX, elapsed);
                if elapsed > 1000 {
                    VERIFY_SIG_SLOW.fetch_add(1, Ordering::Relaxed);
                }
            }
            _ => {}
        }
        if elapsed > 1000 {
            eprintln!("PERF {}: {}us", self.label, elapsed);
        }
    }
}

fn track_max(atom: &AtomicU64, val: u64) {
    let mut current = atom.load(Ordering::Relaxed);
    while val > current {
        match atom.compare_exchange(current, val, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => break,
            Err(actual) => current = actual,
        }
    }
}

static PROCESS_VOTE_COUNT: AtomicU64 = AtomicU64::new(0);
static PROCESS_VOTE_TOTAL: AtomicU64 = AtomicU64::new(0);
static PROCESS_VOTE_MAX: AtomicU64 = AtomicU64::new(0);
static TRY_ADVANCE_COUNT: AtomicU64 = AtomicU64::new(0);
static TRY_ADVANCE_TOTAL: AtomicU64 = AtomicU64::new(0);
static TRY_ADVANCE_MAX: AtomicU64 = AtomicU64::new(0);
static VERIFY_SIG_COUNT: AtomicU64 = AtomicU64::new(0);
static VERIFY_SIG_TOTAL: AtomicU64 = AtomicU64::new(0);
static VERIFY_SIG_MAX: AtomicU64 = AtomicU64::new(0);
static PROCESS_VOTE_SLOW: AtomicU64 = AtomicU64::new(0);
static TRY_ADVANCE_SLOW: AtomicU64 = AtomicU64::new(0);
static VERIFY_SIG_SLOW: AtomicU64 = AtomicU64::new(0);
static PERF_PRINTED: AtomicBool = AtomicBool::new(false);

/// Auto-printing guard — calls perf_summary() on drop.
pub struct PerfSummaryGuard;
impl Drop for PerfSummaryGuard {
    fn drop(&mut self) {
        if !PERF_PRINTED.swap(true, Ordering::SeqCst) {
            perf_summary();
        }
    }
}

pub fn perf_summary() {
    let pv_c = PROCESS_VOTE_COUNT.load(Ordering::Relaxed);
    let pv_t = PROCESS_VOTE_TOTAL.load(Ordering::Relaxed);
    let pv_m = PROCESS_VOTE_MAX.load(Ordering::Relaxed);
    let ta_c = TRY_ADVANCE_COUNT.load(Ordering::Relaxed);
    let ta_t = TRY_ADVANCE_TOTAL.load(Ordering::Relaxed);
    let ta_m = TRY_ADVANCE_MAX.load(Ordering::Relaxed);
    let vs_c = VERIFY_SIG_COUNT.load(Ordering::Relaxed);
    let vs_t = VERIFY_SIG_TOTAL.load(Ordering::Relaxed);
    let vs_m = VERIFY_SIG_MAX.load(Ordering::Relaxed);

    eprintln!("\n=== PERF SUMMARY ===");
    if pv_c > 0 {
        eprintln!(
            "  process_vote:  {} calls, avg {}us, max {}us, slow(>1ms)={}",
            pv_c,
            pv_t / pv_c,
            pv_m,
            PROCESS_VOTE_SLOW.load(Ordering::Relaxed)
        );
    }
    if ta_c > 0 {
        eprintln!(
            "  try_advance:   {} calls, avg {}us, max {}us, slow(>1ms)={}",
            ta_c,
            ta_t / ta_c,
            ta_m,
            TRY_ADVANCE_SLOW.load(Ordering::Relaxed)
        );
    }
    if vs_c > 0 {
        eprintln!(
            "  verify_sig:    {} calls, avg {}us, max {}us, slow(>1ms)={}",
            vs_c,
            vs_t / vs_c,
            vs_m,
            VERIFY_SIG_SLOW.load(Ordering::Relaxed)
        );
    }
    eprintln!("====================\n");
}
