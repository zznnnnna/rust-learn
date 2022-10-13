if [[ $(echo $0 | awk '/^//') == $0 ]]; then
    ABSPATH=$(dirname $0)
else
    ABSPATH=$PWD/$(dirname $0)
fi
cd ${ABSPATH}
#mac-linux
#cargo build --release --target x86_64-unknown-linux-musl
#linux-linux
cargo build --release --target x86_64-unknown-linux-gnu
