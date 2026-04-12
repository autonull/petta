#!/bin/sh

SCRIPT_DIR=$(cd -- "$(dirname -- "$0")" && pwd)

run_test() {
    f="$1"
    echo "Running $f"
    tmpfile="/tmp/petta_test_$$.out"
    "$SCRIPT_DIR/target/release/petta" "$f" > "$tmpfile" 2>&1
    
    output=$(cat "$tmpfile" | grep " should " | grep " is " || echo "")
    rm -f "$tmpfile"
    
    if echo "$output" | grep -q "❌"; then
        echo "FAILURE in $f:"
        echo "$output"
        return 1
    else
        echo "OK: $f"
        echo "$output"
        return 0
    fi
}

status=0
for f in ./examples/*.metta; do
    base=$(basename "$f")
    case "$base" in
        repl.metta|llm_cities.metta|torch.metta|greedy_chess.metta|git_import2.metta|\
        matespacefast.metta)
        echo "Skipping $base"
        ;;
    *)
        run_test "$f" || status=$?
        ;;
    esac
    if [ $status -ne 0 ]; then
        echo ""
        echo "==============================="
        echo "Test failed: $base"
        echo "==============================="
        break
    fi
done

exit $status