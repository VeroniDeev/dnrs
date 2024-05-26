echo "Creating dnrs user and group..."
getent group dnrs >/dev/null 2>&1 || groupadd -r dnrs
getent passwd dnrs >/dev/null 2>&1 || useradd -r -g dnrs -d /etc/dnrs -s /bin/false -c "DNRS user" dnrs
echo "User and group dnrs created."

cargo build --release

install -d "/etc/dnrs/dns"

install -Dm755 "target/release/dnrs" "/etc/dnrs/dnrs"
install -Dm644 LICENSE "/usr/share/licenses/dnrs/LICENSE"
install -Dm644 README.md "/usr/share/doc/dnrs/README.md"

install -Dm644 dnrs.service "/usr/lib/systemd/system/dnrs.service"