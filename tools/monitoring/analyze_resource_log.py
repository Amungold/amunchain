#!/usr/bin/env python3
"""N103 A18‑A20 Resource Log Analyzer — memory growth %, process‑CPU, conn spikes."""
import csv, sys

def analyze(csv_path: str):
    rows = []
    with open(csv_path) as f:
        reader = csv.DictReader((line for line in f if not line.startswith("#")))
        for r in reader:
            rows.append(r)
    if not rows:
        print("ERROR: empty log"); return

    n = len(rows)
    rss_vals  = [int(r["rss_kb"]) for r in rows]
    swap_vals = [int(r["vmswap_kb"]) for r in rows]
    cpu_vals  = [float(r["cpu_pct"]) for r in rows]
    fds_vals  = [int(r["fds"]) for r in rows]
    conn_vals = [int(r["conns"]) for r in rows]
    bpm_vals  = [int(r["blocks_per_min"]) for r in rows]
    disk_data_vals = [int(r["disk_data_kb"]) for r in rows]
    disk_wal_vals  = [int(r["disk_wal_kb"]) for r in rows]

    # A18 — Memory (growth percentage < 10%)
    rss_start, rss_end = rss_vals[0], rss_vals[-1]
    rss_growth_pct = (rss_end - rss_start) / max(rss_start, 1) * 100 if rss_start > 0 else 0
    max_swap = max(swap_vals)
    a18_pass = rss_growth_pct < 10.0 and max_swap == 0
    print(f"A18 Memory:  {'PASS' if a18_pass else 'FAIL'}  "
          f"RSS {rss_start//1024}→{rss_end//1024} MB "
          f"(Δ{rss_growth_pct:.1f}%)  Swap max={max_swap} KB")

    # A19 — CPU (per‑process, multi‑core normalised)
    avg_cpu = sum(cpu_vals)/n if n else 0
    max_cpu = max(cpu_vals)
    a19_pass = avg_cpu < 50 and max_cpu < 150
    print(f"A19 CPU:     {'PASS' if a19_pass else 'FAIL'}  "
          f"avg={avg_cpu:.1f}%  peak={max_cpu:.1f}%  "
          f"blocks/min avg={sum(bpm_vals)//n}")

    # A20 — Disk / FDs / Connections (spike detection)
    disk_data_start, disk_data_end = disk_data_vals[0], disk_data_vals[-1]
    disk_growth_mb = (disk_data_end - disk_data_start) // 1024 if disk_data_end > disk_data_start else 0
    max_fds = max(fds_vals)
    avg_conn = sum(conn_vals)/n if n else 0
    max_conn = max(conn_vals)
    conn_spike = max_conn > avg_conn * 3 and max_conn > 20
    a20_pass = max_fds < 5000 and not conn_spike
    print(f"A20 Disk/FDs:{'PASS' if a20_pass else 'FAIL'}  "
          f"FDs max={max_fds}  Disk Δ{disk_growth_mb} MB  "
          f"conns avg={avg_conn:.1f} max={max_conn} "
          f"({'SPIKE' if conn_spike else 'stable'})")

    overall = a18_pass and a19_pass and a20_pass
    print(f"\nOverall N103 A18‑A20: {'PASS' if overall else 'FAIL'}")
    return overall

if __name__ == "__main__":
    csv_file = sys.argv[1] if len(sys.argv) > 1 else "resource_log.csv"
    ok = analyze(csv_file)
    sys.exit(0 if ok else 1)
