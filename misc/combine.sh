#!/bin/sh

set -e

# Force ASCII collation order to guarantee strict lexicographical sorting of glob expansions
export LC_ALL=C

DIR="${1:-data}"

if [ ! -d "$DIR" ]; then
    echo "Error: Directory '$DIR' does not exist." >&2
    exit 1
fi

OUT_FILE="$DIR/combined.binpack"
TEMP_FILE="$DIR/combined.binpack.tmp"
OUT_META="$DIR/combined.binpack.meta"
TEMP_META="$DIR/combined.binpack.meta.tmp"

rm -f "$TEMP_FILE" "$TEMP_META"

# Ensure the temp files are cleaned up if the script is interrupted or fails
trap 'rm -f "$TEMP_FILE" "$TEMP_META"' EXIT INT TERM

echo "Concatenating .binpack files in '$DIR'..."

COUNT=0

GAMES_COMPLETED=0
TOTAL_POSITIONS=0
WHITE_WINS=0
BLACK_WINS=0
DRAWS=0
HAS_META=0

for file in "$DIR"/*.binpack; do
    if [ ! -f "$file" ]; then
        continue
    fi

    filename=$(basename "$file")
    if [ "$filename" = "combined.binpack" ] || [ "$filename" = "combined.binpack.tmp" ]; then
        continue
    fi

    echo "Appending: $filename"
    cat "$file" >> "$TEMP_FILE"
    COUNT=$((COUNT + 1))

    if [ -f "$file.meta" ]; then
        HAS_META=1
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

        GAMES_COMPLETED=$((GAMES_COMPLETED + val_gc))
        TOTAL_POSITIONS=$((TOTAL_POSITIONS + val_tp))
        WHITE_WINS=$((WHITE_WINS + val_ww))
        BLACK_WINS=$((BLACK_WINS + val_bw))
        DRAWS=$((DRAWS + val_dr))
    fi
done

if [ "$COUNT" -eq 0 ]; then
    echo "No .binpack files found in '$DIR'."
    exit 0
fi

if [ "$HAS_META" -eq 1 ]; then
    printf '{\n  "games_completed": %d,\n  "total_positions": %d,\n  "white_wins": %d,\n  "black_wins": %d,\n  "draws": %d\n}\n' \
        "$GAMES_COMPLETED" "$TOTAL_POSITIONS" "$WHITE_WINS" "$BLACK_WINS" "$DRAWS" > "$TEMP_META"
fi

mv "$TEMP_FILE" "$OUT_FILE"
if [ "$HAS_META" -eq 1 ]; then
    mv "$TEMP_META" "$OUT_META"
fi

trap - EXIT INT TERM

if [ "$HAS_META" -eq 1 ]; then
    echo "Successfully combined $COUNT files into '$OUT_FILE' and its metadata into '$OUT_META'."
else
    echo "Successfully combined $COUNT files into '$OUT_FILE'."
fi
