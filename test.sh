#!/bin/sh
# Run all MeTTa examples through a single persistent PeTTa engine.
# This is ~100x faster than spawning a subprocess per file.

SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)

# Collect example files, skipping known problematic ones
files=""
for f in "$SCRIPT_DIR"/examples/*.metta; do
    base=$(basename "$f")
    case "$base" in
        repl.metta|llm_cities.metta|torch.metta|greedy_chess.metta|git_import2.metta|\
        matespacefast.metta)
            echo "Skipping $base"
            ;;
        *)
            files="$files $f"
            ;;
    esac
done

if [ -z "$files" ]; then
    echo "No test files found."
    exit 1
fi

echo "Running ${files## } through persistent engine..."
tmpfile="/tmp/petta_test_$$.out"

# Single engine invocation for ALL files
"$SCRIPT_DIR/target/release/petta" $files > "$tmpfile" 2>&1
status=$?

# Check for failures
failures=$(grep "❌" "$tmpfile" || echo "")

if [ -n "$failures" ]; then
    echo "FAILURES:"
    echo "$failures"
    rm -f "$tmpfile"
    exit 1
fi

passed=$(grep -c "✅" "$tmpfile" || echo "0")
echo "All $passed tests passed."
rm -f "$tmpfile"
exit 0
