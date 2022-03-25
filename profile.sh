#!/bin/bash
mkdir -p profiles
NAME="profiles/gateway-$1"
USR="${SUDO_USER:-$USER}"
GRP="${SUDO_USER:-$(whoami)}"
perf record --pid=$(pgrep gateway) --output="$NAME".data --freq=1009 -g -- sleep 60
perf script --input="$NAME".data | ../FlameGraph/stackcollapse-perf.pl > "$NAME"-out.perf-folded
cat "$NAME"-out.perf-folded | ../FlameGraph/flamegraph.pl > "$NAME".svg
chown "$USR":"$GRP" profiles/*