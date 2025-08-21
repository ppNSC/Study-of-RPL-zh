# download rustup
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

# source env
. ~/.cargo/env

# switch rustc version(such as 1.85.0)
rustup install 1.85.0
rustup default 1.85.0

# update and uninstall
rustup update
rustup self uninstall
