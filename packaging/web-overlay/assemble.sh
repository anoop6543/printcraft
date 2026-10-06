#!/usr/bin/env bash
# Assemble the Split Happens static site from a trunk build + the web overlay.
#
# Usage: packaging/web-overlay/assemble.sh
# Input:  dist/printcraft-web-*.zip (from packaging/web/package.sh)
#         packaging/web-overlay/site/ (loader template, PWA files, mobile app)
# Output: /tmp/split-happens-site/ ready to publish (e.g. GitHub Pages).
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../.." && pwd)"
OVERLAY="$HERE/site"
OUT="/tmp/split-happens-site"

ZIP="$(ls -t "$ROOT"/dist/release/printcraft-web-*.zip 2>/dev/null | head -1)"
if [ -z "$ZIP" ]; then
  ZIP="$(ls -t "$ROOT"/dist/printcraft-web-*.zip 2>/dev/null | head -1)"
fi
[ -n "$ZIP" ] || { echo "error: no printcraft-web-*.zip in $ROOT/dist/release or $ROOT/dist" >&2; exit 1; }
echo "using $ZIP"
rm -rf /tmp/webdist "$OUT"
mkdir -p /tmp/webdist "$OUT"
unzip -q "$ZIP" -d /tmp/webdist
INNER="$(ls -d /tmp/webdist/printcraft-web-*/ 2>/dev/null | head -1)"
[ -n "$INNER" ] || { echo "error: no printcraft-web-*/ dir in /tmp/webdist" >&2; exit 1; }
TRUNK_INDEX="$INNER/index.html"

# Extract the hashed trunk asset names + integrity hashes from trunk's index.html.
JS="$(grep -oE 'href="\./printcraft-web-[a-f0-9]+\.js"' "$TRUNK_INDEX" | head -1 | cut -d'"' -f2 | cut -c3-)"
WASM="$(grep -oE 'href="\./printcraft-web-[a-f0-9]+_bg\.wasm"' "$TRUNK_INDEX" | head -1 | cut -d'"' -f2 | cut -c3-)"
[ -n "$JS" ] && [ -n "$WASM" ] || { echo "error: could not find trunk assets in $TRUNK_INDEX" >&2; exit 1; }
echo "js:   $JS"
echo "wasm: $WASM"

# Copy trunk outputs, then overlay our shell (loader page, PWA, mobile app).
cp "$INNER/$JS" "$INNER/$WASM" "$OUT/"
cp -R "$OVERLAY/." "$OUT/"
rm "$OUT/index.template.html" "$OUT/sw.template.js"

# Render templates with the real filenames and fresh SRI hashes.
JS_INT="sha384-$(openssl dgst -sha384 -binary "$OUT/$JS" | openssl base64 -A)"
WASM_INT="sha384-$(openssl dgst -sha384 -binary "$OUT/$WASM" | openssl base64 -A)"
sed -e "s|%%JS%%|$JS|g" -e "s|%%WASM%%|$WASM|g" \
    -e "s|%%JS_INT%%|$JS_INT|g" -e "s|%%WASM_INT%%|$WASM_INT|g" \
    "$OVERLAY/index.template.html" > "$OUT/index.html"
sed -e "s|%%JS%%|$JS|g" -e "s|%%WASM%%|$WASM|g" \
    "$OVERLAY/sw.template.js" > "$OUT/sw.js"

echo "site assembled in $OUT:"
ls -la "$OUT" | head -20
