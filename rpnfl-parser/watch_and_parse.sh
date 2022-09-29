while inotifywait -qq -e close_write "$1"; do ./target/debug/rpnfl-parser "$1"; done
