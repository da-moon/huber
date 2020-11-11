#!/usr/bin/env bash

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

function install_linux_dependencies() {
  if [[ $(command -v apt) ]]; then
    sudo apt update
    sudo apt install -y build-essential libssl-dev libarchive-dev git pkg-config curl
  elif [[ $(command -v zypper) ]]; then
    sudo zypper install -y -t pattern devel_basis
    sudo zypper install -y libopenssl-devel libarchive-devel git pkg-config curl
  else
    echo "Only openSUSE, Ubuntu supported" >/dev/stderr
    exit 1
  fi
}

function install_macos_dependencies() {
  if [[ ! $(command -v brew) ]]; then
    curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh | sh -
  fi

  # https://github.com/libarchive/libarchive/blob/master/.github/workflows/ci.yml
  brew uninstall openssl@1.0.2t
  brew uninstall python@2.7.17
  brew untap local/openssl
  brew untap local/python2
  brew update
  # brew upgrade

  for pkg in \
    autoconf \
    automake \
    libtool \
    pkg-config \
    cmake \
    xz \
    lz4 \
    zstd \
    openssl \
    libarchive; do
    (brew list $pkg && brew upgrade $pkg) || brew install $pkg
  done

  {
    echo "export PATH=/usr/local/opt/libarchive/bin:\$PATH"
    echo "export LDFLAGS=-L/usr/local/opt/libarchive/lib"
    echo "export CPPFLAGS=-I/usr/local/opt/libarchive/include"
    echo "export PKG_CONFIG_PATH=/usr/local/opt/libarchive/lib/pkgconfig"
  } >> "$HOME"/.bashrc

  # shellcheck disable=SC1090
  . "$HOME"/.bashrc
}

function install_rust_dependencies() {
  if [[ -z $(command -v cargo 2>/dev/null) ]]; then
    curl https://sh.rustup.rs -sSf | sh
  fi
  cargo version

  echo "export PATH=\$HOME/.cargo/bin:\$PATH" >>"$HOME"/.bashrc
  # shellcheck disable=SC1090
  . "$HOME"/.bashrc
}

os=$(uname)
case $os in
"Linux")
  install_linux_dependencies
  ;;
"Darwin")
  install_macos_dependencies
  ;;
esac

install_rust_dependencies