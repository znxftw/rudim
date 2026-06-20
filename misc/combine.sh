#!/bin/bash

set -e

# Force ASCII collation order to guarantee strict lexicographical sorting of glob expansions
export LC_ALL=C

INPUT_VERSION="${1:-v3}"
DIR="${2:-data}"

if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist." >&2
    exit 1
fi

# Parse target version number
if [[ "$INPUT_VERSION" =~ ^v?([0-9]+)$ ]]; then
    TARGET_VER="${BASH_REMATCH[1]}"
else
    echo "Error: Invalid version format '$INPUT_VERSION'. Expected e.g. v3 or 3." >&2
    exit 1
fi

ALL_OUT_FILE="$DIR/all.binpack"
ALL_TEMP_FILE="$DIR/all.binpack.tmp"
ALL_OUT_META="$DIR/all.binpack.meta"
ALL_TEMP_META="$DIR/all.binpack.meta.tmp"

LATEST_OUT_FILE="$DIR/latest.binpack"
LATEST_TEMP_FILE="$DIR/latest.binpack.tmp"
LATEST_OUT_META="$DIR/latest.binpack.meta"
LATEST_TEMP_META="$DIR/latest.binpack.meta.tmp"

rm -f "$ALL_TEMP_FILE" "$ALL_TEMP_META" "$LATEST_TEMP_FILE" "$LATEST_TEMP_META"

# Ensure the temp files are cleaned up if the script is interrupted or fails
trap 'rm -f "$ALL_TEMP_FILE" "$ALL_TEMP_META" "$LATEST_TEMP_FILE" "$LATEST_TEMP_META"' EXIT INT TERM

echo "Processing .binpack files in '$DIR' (Target version >= v$TARGET_VER for latest.binpack)..."

ALL_COUNT=0
ALL_GAMES_COMPLETED=0
ALL_TOTAL_POSITIONS=0
ALL_WHITE_WINS=0
ALL_BLACK_WINS=0
ALL_DRAWS=0
ALL_HAS_META=0

LATEST_COUNT=0
LATEST_GAMES_COMPLETED=0
LATEST_TOTAL_POSITIONS=0
LATEST_WHITE_WINS=0
LATEST_BLACK_WINS=0
LATEST_DRAWS=0
LATEST_HAS_META=0

for file in "$DIR"/*.binpack; do
    if [ ! -f "$file" ]; then
        continue
    fi

    filename=$(basename "$file")
    # Skip any output files or temporary files
    if [[ "$filename" == "all.binpack"* ]] || [[ "$filename" == "latest.binpack"* ]] || [[ "$filename" == "combined.binpack"* ]]; then
        continue
    fi

    # Extract version number if the filename starts with v<number>
    file_version=""
    if [[ "$filename" =~ ^v([0-9]+) ]]; then
        file_version="${BASH_REMATCH[1]}"
    fi

    # Parse meta values if meta file exists
    val_gc=0
    val_tp=0
    val_ww=0
    val_bw=0
    val_dr=0
    has_this_meta=0

    if [ -f "$file.meta" ]; then
        has_this_meta=1
        vals=$(awk '
        /"games_completed"/ { gsub(/[^0-9]/, "", $0); gc = $0 }
        /"total_positions"/ { gsub(/[^0-9]/, "", $0); tp = $0 }
        /"white_wins"/      { gsub(/[^0-9]/, "", $0); ww = $0 }
        /"black_wins"/      { gsub(/[^0-9]/, "", $0); bw = $0 }
        /"draws"/           { gsub(/[^0-9]/, "", $0); dr = $0 }
        END { print gc+0, tp+0, ww+0, bw+0, dr+0 }
        ' "$file.meta")

        read -r val_gc val_tp val_ww val_bw val_dr <<EOF
$vals
EOF
    fi

    # Append to all.binpack
    echo "Appending to all.binpack: $filename"
    cat "$file" >> "$ALL_TEMP_FILE"
    ALL_COUNT=$((ALL_COUNT + 1))
    if [ "$has_this_meta" -eq 1 ]; then
        ALL_HAS_META=1
        ALL_GAMES_COMPLETED=$((ALL_GAMES_COMPLETED + val_gc))
        ALL_TOTAL_POSITIONS=$((ALL_TOTAL_POSITIONS + val_tp))
        ALL_WHITE_WINS=$((ALL_WHITE_WINS + val_ww))
        ALL_BLACK_WINS=$((ALL_BLACK_WINS + val_bw))
        ALL_DRAWS=$((ALL_DRAWS + val_dr))
    fi

    # Check if this file qualifies for latest.binpack
    if [ -n "$file_version" ] && [ "$file_version" -ge "$TARGET_VER" ]; then
        echo "Appending to latest.binpack: $filename"
        cat "$file" >> "$LATEST_TEMP_FILE"
        LATEST_COUNT=$((LATEST_COUNT + 1))
        if [ "$has_this_meta" -eq 1 ]; then
            LATEST_HAS_META=1
            LATEST_GAMES_COMPLETED=$((LATEST_GAMES_COMPLETED + val_gc))
            LATEST_TOTAL_POSITIONS=$((LATEST_TOTAL_POSITIONS + val_tp))
            LATEST_WHITE_WINS=$((LATEST_WHITE_WINS + val_ww))
            LATEST_BLACK_WINS=$((LATEST_BLACK_WINS + val_bw))
            LATEST_DRAWS=$((LATEST_DRAWS + val_dr))
        fi
    fi
done

if [ "$ALL_COUNT" -eq 0 ]; then
    echo "No .binpack files found in '$DIR'."
    exit 0
fi

# Write metadata files if they exist
if [ "$ALL_HAS_META" -eq 1 ]; then
    printf '{\n  "games_completed": %d,\n  "total_positions": %d,\n  "white_wins": %d,\n  "black_wins": %d,\n  "draws": %d\n}\n' \
        "$ALL_GAMES_COMPLETED" "$ALL_TOTAL_POSITIONS" "$ALL_WHITE_WINS" "$ALL_BLACK_WINS" "$ALL_DRAWS" > "$ALL_TEMP_META"
fi

if [ "$LATEST_COUNT" -gt 0 ] && [ "$LATEST_HAS_META" -eq 1 ]; then
    printf '{\n  "games_completed": %d,\n  "total_positions": %d,\n  "white_wins": %d,\n  "black_wins": %d,\n  "draws": %d\n}\n' \
        "$LATEST_GAMES_COMPLETED" "$LATEST_TOTAL_POSITIONS" "$LATEST_WHITE_WINS" "$LATEST_BLACK_WINS" "$LATEST_DRAWS" > "$LATEST_TEMP_META"
fi

# Final move of temp files to output files
mv "$ALL_TEMP_FILE" "$ALL_OUT_FILE"
if [ "$ALL_HAS_META" -eq 1 ]; then
    mv "$ALL_TEMP_META" "$ALL_OUT_META"
fi

if [ "$LATEST_COUNT" -gt 0 ]; then
    mv "$LATEST_TEMP_FILE" "$LATEST_OUT_FILE"
    if [ "$LATEST_HAS_META" -eq 1 ]; then
        mv "$LATEST_TEMP_META" "$LATEST_OUT_META"
    fi
else
    echo "No files matched version >= v$TARGET_VER; '$LATEST_OUT_FILE' was not created/updated."
    rm -f "$LATEST_OUT_FILE" "$LATEST_OUT_META"
fi

trap - EXIT INT TERM

echo ""
echo "Summary:"
if [ "$ALL_HAS_META" -eq 1 ]; then
    echo "  all.binpack: Combined $ALL_COUNT files (with metadata)."
else
    echo "  all.binpack: Combined $ALL_COUNT files (no metadata)."
fi

if [ "$LATEST_COUNT" -gt 0 ]; then
    if [ "$LATEST_HAS_META" -eq 1 ]; then
        echo "  latest.binpack: Combined $LATEST_COUNT files (with metadata)."
    else
        echo "  latest.binpack: Combined $LATEST_COUNT files (no metadata)."
    fi
else
    echo "  latest.binpack: No files combined."
fi
