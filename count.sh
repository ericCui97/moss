#!/bin/bash
# 统计有多少行的rust代码
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

dir="$1"

count_rust_lines() {
  local total_lines=0

  for entry in "$1"/*; do
    if [ -d "$entry" ]; then
      total_lines=$((total_lines + $(count_rust_lines "$entry")))
    elif [ "${entry##*.}" == "rs" ]; then
      total_lines=$((total_lines + $(wc -l < "$entry")))
    fi
  done

  echo $total_lines
}

total_lines=$(count_rust_lines "$dir")
echo "Total Rust code lines: $total_lines"