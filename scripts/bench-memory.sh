#!/usr/bin/env bash
set -euo pipefail

package="${PACKAGE:-cast-gallery}"
binary="${BINARY:-target/release/cast-gallery}"
out_dir="${OUT_DIR:-.workspace/benchmarks}"
samples="${SAMPLES:-12}"
interval="${INTERVAL:-1}"
warmup="${WARMUP:-3}"
build="${BUILD:-1}"

usage() {
  cat <<'USAGE'
Usage: scripts/bench-memory.sh

Environment overrides:
  PACKAGE   Cargo package to build before running. Default: cast-gallery
  BINARY    Binary path to launch. Default: target/release/cast-gallery
  OUT_DIR   Output directory. Default: .workspace/benchmarks
  SAMPLES   Number of samples after warmup. Default: 12
  INTERVAL  Seconds between samples. Default: 1
  WARMUP    Seconds to wait before sampling. Default: 3
  BUILD     Set to 0 to skip cargo build. Default: 1
USAGE
}

if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
  usage
  exit 0
fi

if [[ "$build" != "0" ]]; then
  cargo build --release -p "$package"
fi

mkdir -p "$out_dir"
timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="$out_dir/$timestamp-$package"
mkdir -p "$run_dir"
csv="$run_dir/memory.csv"
report="$run_dir/report.md"
app_log="$run_dir/app.log"

"$binary" >"$app_log" 2>&1 &
pid="$!"

cleanup() {
  if kill -0 "$pid" 2>/dev/null; then
    kill "$pid" 2>/dev/null || true
    sleep 1
    kill -9 "$pid" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

sleep "$warmup"

if [[ ! -r "/proc/$pid/smaps_rollup" ]]; then
  echo "Cannot read /proc/$pid/smaps_rollup; is the process still running?" >&2
  exit 1
fi

printf 'sample,elapsed_s,vmrss_kib,rss_kib,pss_kib,private_clean_kib,private_dirty_kib,private_total_kib,shared_clean_kib,shared_dirty_kib,swap_kib\n' >"$csv"

sample_smaps() {
  local sample="$1"
  local elapsed="$2"
  local vmrss
  vmrss="$(awk '/^VmRSS:/ {print $2}' "/proc/$pid/status")"
  awk -v sample="$sample" -v elapsed="$elapsed" -v vmrss="$vmrss" '
    /^(Rss|Pss|Private_Clean|Private_Dirty|Shared_Clean|Shared_Dirty|Swap):/ {
      key = $1
      sub(":", "", key)
      value[key] = $2
    }
    END {
      private_total = value["Private_Clean"] + value["Private_Dirty"]
      printf "%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\n",
        sample,
        elapsed,
        vmrss,
        value["Rss"],
        value["Pss"],
        value["Private_Clean"],
        value["Private_Dirty"],
        private_total,
        value["Shared_Clean"],
        value["Shared_Dirty"],
        value["Swap"]
    }
  ' "/proc/$pid/smaps_rollup" >>"$csv"
}

for ((sample = 1; sample <= samples; sample++)); do
  if [[ ! -r "/proc/$pid/smaps_rollup" ]]; then
    echo "Process exited before sample $sample" >&2
    break
  fi
  sample_smaps "$sample" "$((sample * interval))"
  sleep "$interval"
done

binary_size_bytes="unknown"
if [[ -f "$binary" ]]; then
  binary_size_bytes="$(stat -c '%s' "$binary")"
fi

summary="$(
  awk -F, '
    NR == 1 { next }
    {
      count += 1
      rss += $4
      pss += $5
      private_total += $8
      final_rss = $4
      final_pss = $5
      final_private_total = $8
      if (count == 1 || $4 > peak_rss) peak_rss = $4
      if (count == 1 || $5 > peak_pss) peak_pss = $5
      if (count == 1 || $8 > peak_private_total) peak_private_total = $8
    }
    END {
      if (count == 0) {
        print "samples=0"
      } else {
        printf "samples=%d\navg_rss_kib=%.0f\navg_pss_kib=%.0f\navg_private_total_kib=%.0f\npeak_rss_kib=%d\npeak_pss_kib=%d\npeak_private_total_kib=%d\nfinal_rss_kib=%d\nfinal_pss_kib=%d\nfinal_private_total_kib=%d\n",
          count,
          rss / count,
          pss / count,
          private_total / count,
          peak_rss,
          peak_pss,
          peak_private_total,
          final_rss,
          final_pss,
          final_private_total
      }
    }
  ' "$csv"
)"

{
  echo "# Memory Benchmark"
  echo
  echo "- UTC timestamp: $timestamp"
  echo "- Package: $package"
  echo "- Binary: $binary"
  echo "- Binary size bytes: $binary_size_bytes"
  echo "- Warmup seconds: $warmup"
  echo "- Samples: $samples"
  echo "- Interval seconds: $interval"
  echo "- CSV: memory.csv"
  echo "- App log: app.log"
  echo
  echo "## Summary"
  echo
  while IFS='=' read -r key value; do
    [[ -z "$key" ]] && continue
    echo "- $key: $value"
  done <<<"$summary"
  echo
  echo "## Notes"
  echo
  echo "RSS includes shared mappings. PSS and private_total_kib are usually better for comparing desktop UI stacks."
} >"$report"

cleanup
trap - EXIT INT TERM

echo "Wrote $report"
