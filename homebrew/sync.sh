#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_SH="$SCRIPT_DIR/install.sh"

if [[ ! -f "$INSTALL_SH" ]]; then
    echo "Error: $INSTALL_SH not found"
    exit 1
fi

extract_array() {
    awk -v name="$1" '
        $0 ~ "^"name"=\\(" { in_array=1; next }
        in_array && /^\)/ { exit }
        in_array {
            sub(/#.*/, "")
            gsub(/^[[:space:]]+|[[:space:]]+$/, "")
            if ($0 != "") print
        }
    ' "$INSTALL_SH"
}

add_to_array() {
    local tmp
    tmp=$(mktemp)
    awk -v name="$1" -v line="$2" '
        $0 ~ "^"name"=\\(" { in_array=1; print; next }
        in_array && /^\)/ { print line; in_array=0 }
        { print }
    ' "$INSTALL_SH" > "$tmp" && mv "$tmp" "$INSTALL_SH"
}

contains() {
    local needle="$1"
    shift
    local item
    for item in "$@"; do
        [[ "$item" == "$needle" ]] && return 0
    done
    return 1
}

config_formulae=()
while IFS= read -r line; do
    [[ -n "$line" ]] && config_formulae+=("$line")
done < <(extract_array FORMULAE)

config_casks=()
while IFS= read -r line; do
    [[ -n "$line" ]] && config_casks+=("$line")
done < <(extract_array CASKS)

installed_formulae=()
while IFS= read -r line; do
    [[ -n "$line" ]] && installed_formulae+=("$line")
done < <(brew leaves)

installed_casks=()
while IFS= read -r line; do
    [[ -n "$line" ]] && installed_casks+=("$line")
done < <(brew list --cask)

new_formulae=()
for f in "${installed_formulae[@]}"; do
    contains "$f" "${config_formulae[@]}" || new_formulae+=("$f")
done

new_casks=()
for c in "${installed_casks[@]}"; do
    contains "$c" "${config_casks[@]}" || new_casks+=("$c")
done

total=$((${#new_formulae[@]} + ${#new_casks[@]}))

if [[ $total -eq 0 ]]; then
    echo "In sync: no new packages"
    exit 0
fi

echo "Found $total new package(s): ${#new_formulae[@]} formulae, ${#new_casks[@]} casks"
echo

added=0
skipped=0

prompt_add() {
    local kind="$1" pkg="$2" array_name="$3" ans comment
    while true; do
        printf "Add %s '%s'? [y/n/c/q] " "$kind" "$pkg"
        read -r ans < /dev/tty
        case "$ans" in
            y|Y)
                add_to_array "$array_name" "    $pkg"
                echo "  added"
                added=$((added + 1))
                return 0
                ;;
            c|C)
                printf "  comment: "
                read -r comment < /dev/tty
                add_to_array "$array_name" "    $pkg    # $comment"
                echo "  added"
                added=$((added + 1))
                return 0
                ;;
            n|N)
                echo "  skipped"
                skipped=$((skipped + 1))
                return 0
                ;;
            q|Q)
                return 1
                ;;
            *)
                echo "  [y]es [n]o [c]omment-then-add [q]uit"
                ;;
        esac
    done
}

quit=0
for f in "${new_formulae[@]}"; do
    prompt_add formula "$f" FORMULAE || { quit=1; break; }
done

if [[ $quit -eq 0 ]]; then
    for c in "${new_casks[@]}"; do
        prompt_add cask "$c" CASKS || { quit=1; break; }
    done
fi

echo
echo "Done: $added added, $skipped skipped"
