#!/usr/bin/env bash
set -euo pipefail

package="${PACKAGE:-cast-gallery}"
binary="${BINARY:-target/release/cast-gallery}"
out_dir="${OUT_DIR:-.workspace/benchmarks}"
warmup="${WARMUP:-5}"
top="${TOP:-30}"
build="${BUILD:-1}"

usage() {
  cat <<'USAGE'
Usage: scripts/bench-smaps-breakdown.sh

Captures /proc/$pid/smaps for a release desktop run and groups memory by
mapping path and broad category.

Environment overrides:
  PACKAGE   Cargo package to build before running. Default: cast-gallery
  BINARY    Binary path to launch. Default: target/release/cast-gallery
  OUT_DIR   Output directory. Default: .workspace/benchmarks
  WARMUP    Seconds to wait before capturing smaps. Default: 5
  TOP       Number of largest mappings to include in report. Default: 30
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
run_dir="$out_dir/$timestamp-$package-smaps"
mkdir -p "$run_dir"
smaps="$run_dir/smaps.txt"
paths_csv="$run_dir/smaps-by-path.csv"
categories_csv="$run_dir/smaps-by-category.csv"
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

if [[ ! -r "/proc/$pid/smaps" ]]; then
  echo "Cannot read /proc/$pid/smaps; is the process still running?" >&2
  exit 1
fi

cp "/proc/$pid/smaps" "$smaps"
cp "/proc/$pid/smaps_rollup" "$run_dir/smaps_rollup.txt"
cp "/proc/$pid/status" "$run_dir/status.txt"

binary_path="$(readlink -f "$binary")"

awk '
  function classify(path) {
    if (path == "" || path == "[anonymous]") return "anonymous"
    if (path ~ /^\[heap\]/) return "heap"
    if (path ~ /^\[stack/) return "stack"
    if (path ~ /^\[/) return "kernel-special"
    if (path ~ /^\/dev\/dri/ || path ~ /^\/dev\/shm/ || path ~ /memfd/ || path ~ /anon_inode/) return "gpu-or-shared-memory"
    if (path ~ /libLLVM/ || path ~ /libgallium/ || path ~ /libvulkan/ || path ~ /libVkLayer/ || path ~ /libdrm/ || path ~ /libGL/ || path ~ /libEGL/ || path ~ /mesa/ || path ~ /vulkan/) return "renderer-driver"
    if (path == binary) return "app-binary"
    if (path ~ /\.ttf$/ || path ~ /\.otf$/ || path ~ /fonts/) return "font-files"
    if (path ~ /\.so/ || path ~ /\/lib/ || path ~ /\/usr\/lib/) return "shared-libraries"
    return "files"
  }

  function flush() {
    if (!seen) return
    if (path == "") path = "[anonymous]"
    category = classify(path)
    size_by_path[path] += size
    rss_by_path[path] += rss
    pss_by_path[path] += pss
    private_by_path[path] += private_clean + private_dirty
    shared_by_path[path] += shared_clean + shared_dirty

    size_by_category[category] += size
    rss_by_category[category] += rss
    pss_by_category[category] += pss
    private_by_category[category] += private_clean + private_dirty
    shared_by_category[category] += shared_clean + shared_dirty

    size = rss = pss = private_clean = private_dirty = shared_clean = shared_dirty = 0
  }

  /^[0-9a-f]+-[0-9a-f]+ / {
    flush()
    seen = 1
    path = ""
    if (NF >= 6) {
      for (i = 6; i <= NF; i++) {
        path = path (i == 6 ? "" : " ") $i
      }
    }
    next
  }

  /^Size:/ { size = $2; next }
  /^Rss:/ { rss = $2; next }
  /^Pss:/ { pss = $2; next }
  /^Private_Clean:/ { private_clean = $2; next }
  /^Private_Dirty:/ { private_dirty = $2; next }
  /^Shared_Clean:/ { shared_clean = $2; next }
  /^Shared_Dirty:/ { shared_dirty = $2; next }

  END {
    flush()
    print "category,size_kib,rss_kib,pss_kib,private_kib,shared_kib" > categories
    for (category in pss_by_category) {
      printf "%s,%d,%d,%d,%d,%d\n",
        category,
        size_by_category[category],
        rss_by_category[category],
        pss_by_category[category],
        private_by_category[category],
        shared_by_category[category] >> categories
    }

    print "path,size_kib,rss_kib,pss_kib,private_kib,shared_kib" > paths
    for (path in pss_by_path) {
      safe_path = path
      gsub(/"/, "\"\"", safe_path)
      printf "\"%s\",%d,%d,%d,%d,%d\n",
        safe_path,
        size_by_path[path],
        rss_by_path[path],
        pss_by_path[path],
        private_by_path[path],
        shared_by_path[path] >> paths
    }
  }
' categories="$categories_csv" paths="$paths_csv" binary="$binary_path" "$smaps"

{
  head -n 1 "$paths_csv"
  tail -n +2 "$paths_csv" | sort -t, -k4,4nr
} >"$run_dir/smaps-by-path.sorted.csv"

{
  head -n 1 "$categories_csv"
  tail -n +2 "$categories_csv" | sort -t, -k4,4nr
} >"$run_dir/smaps-by-category.sorted.csv"

binary_size_bytes="unknown"
if [[ -f "$binary" ]]; then
  binary_size_bytes="$(stat -c '%s' "$binary")"
fi

rollup_summary="$(
  awk '
    /^(Rss|Pss|Private_Clean|Private_Dirty|Shared_Clean|Shared_Dirty|Swap):/ {
      key = $1
      sub(":", "", key)
      value[key] = $2
    }
    END {
      printf "rss_kib=%d\npss_kib=%d\nprivate_total_kib=%d\nshared_total_kib=%d\nswap_kib=%d\n",
        value["Rss"],
        value["Pss"],
        value["Private_Clean"] + value["Private_Dirty"],
        value["Shared_Clean"] + value["Shared_Dirty"],
        value["Swap"]
    }
  ' "$run_dir/smaps_rollup.txt"
)"

{
  echo "# smaps Breakdown"
  echo
  echo "- UTC timestamp: $timestamp"
  echo "- Package: $package"
  echo "- Binary: $binary"
  echo "- Binary size bytes: $binary_size_bytes"
  echo "- Warmup seconds: $warmup"
  echo "- Raw smaps: smaps.txt"
  echo "- Rollup: smaps_rollup.txt"
  echo "- Path CSV: smaps-by-path.sorted.csv"
  echo "- Category CSV: smaps-by-category.sorted.csv"
  echo
  echo "## Rollup"
  echo
  while IFS='=' read -r key value; do
    [[ -z "$key" ]] && continue
    echo "- $key: $value"
  done <<<"$rollup_summary"
  echo
  echo "## Categories By PSS"
  echo
  echo "| Category | Size KiB | RSS KiB | PSS KiB | Private KiB | Shared KiB |"
  echo "| --- | ---: | ---: | ---: | ---: | ---: |"
  tail -n +2 "$run_dir/smaps-by-category.sorted.csv" | while IFS=, read -r category size rss pss private shared; do
    echo "| $category | $size | $rss | $pss | $private | $shared |"
  done
  echo
  echo "## Top Mappings By PSS"
  echo
  echo "| Path | Size KiB | RSS KiB | PSS KiB | Private KiB | Shared KiB |"
  echo "| --- | ---: | ---: | ---: | ---: | ---: |"
  tail -n +2 "$run_dir/smaps-by-path.sorted.csv" | head -n "$top" | while IFS=, read -r path size rss pss private shared; do
    path="${path%\"}"
    path="${path#\"}"
    echo "| \`$path\` | $size | $rss | $pss | $private | $shared |"
  done
} >"$report"

cleanup
trap - EXIT INT TERM

echo "Wrote $report"
