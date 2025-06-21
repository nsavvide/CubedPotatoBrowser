#!/bin/bash

# Ensure smem is installed
command -v smem >/dev/null 2>&1 || { echo "smem not found. Install it with 'paru -S smem'"; exit 1; }

# Function to kill all Chromium instances
kill_chromium() {
    pkill -x chromium
}

# Function to reset the Potato Browser
reset_potato() {
    # optionally send an IPC command to close windows or restart the server if supported
    echo "Ensure Potato Browser is running before testing"
}

echo "============================="
echo "Benchmark: Potato Browser (10 windows)"
echo "============================="
reset_potato
for i in {1..10}; do
    cargo run --bin client OpenNewWindow
    sleep 0.3
done
echo
echo "[Potato Browser Memory]"
smem -r -n -P potato | tee potato_mem.txt

echo
echo "============================="
echo "Benchmark: Chromium (10 windows)"
echo "============================="
kill_chromium
sleep 2  # Give time to clean up

for i in {1..10}; do
  chromium --user-data-dir="/tmp/chrome-profile-$i" --new-window https://www.google.com &
  sleep 1
done
echo
echo "[Chromium Memory]"
smem -r -n -P chromium | tee chromium_mem.txt

echo
echo "✅ Benchmark complete. Results saved to potato_mem.txt and chromium_mem.txt"

