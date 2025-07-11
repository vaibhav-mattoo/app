#!/bin/sh
# The official alman installer
# Supports Linux, macOS, BSD, Windows (MSYS2/Git Bash/Cygwin), all major architectures.

# GitHub repository settings
REPO_OWNER="vaibhav-mattoo"
REPO_NAME="alman"

main() {
    if [ "${KSH_VERSION-}" = 'Version JM 93t+ 2010-03-05' ]; then
        err 'the installer does not work with this ksh93 version; please try bash'
    fi

    set -u
    parse_args "$@"

    local _arch
    _arch="${ARCH:-$(ensure get_architecture)}"
    assert_nz "${_arch}" "arch"
    echo "Detected architecture: ${_arch}"

    local _bin_name
    case "${_arch}" in
    *windows*) _bin_name="${REPO_NAME}.exe" ;;
    *)          _bin_name="${REPO_NAME}" ;;
    esac

    local _tmp_dir
    _tmp_dir="$(mktemp -d)" || err "mktemp: could not create temporary directory"
    cd "${_tmp_dir}" || err "cd: failed to enter directory: ${_tmp_dir}"

    local _package
    _package="$(ensure download_alman "${_arch}")"
    assert_nz "${_package}" "package"
    echo "Downloaded package: ${_package}"
    case "${_package}" in
    *.tar.gz)
        need_cmd tar
        ensure tar -xf "${_package}"
        ;;
    *.zip)
        need_cmd unzip
        ensure unzip -oq "${_package}"
        ;;
    *)
        err "unsupported package format: ${_package}"
        ;;
    esac

    ensure try_sudo mkdir -p -- "${BIN_DIR}"
    ensure try_sudo cp -- "${_bin_name}" "${BIN_DIR}/${_bin_name}"
    ensure try_sudo chmod +x "${BIN_DIR}/${_bin_name}"
    echo "Installed ${REPO_NAME} to ${BIN_DIR}"

    # Install manpages if present
    if [ -d "man/man1" ]; then
        ensure try_sudo mkdir -p -- "${MAN_DIR}/man1"
        ensure try_sudo cp -- "man/man1/"* "${MAN_DIR}/man1/"
        echo "Installed manpages to ${MAN_DIR}"
    fi

    echo ""
    echo "${REPO_NAME} is installed!"
    if ! echo ":${PATH}:" | grep -Fq ":${BIN_DIR}:"; then
        echo "Note: ${BIN_DIR} is not on your \$PATH. ${REPO_NAME} will not work unless it is added to \$PATH."
    fi
}

parse_args() {
    BIN_DIR_DEFAULT="${HOME}/.local/bin"
    MAN_DIR_DEFAULT="${HOME}/.local/share/man"
    SUDO_DEFAULT="sudo"

    BIN_DIR="${BIN_DIR_DEFAULT}"
    MAN_DIR="${MAN_DIR_DEFAULT}"
    SUDO="${SUDO_DEFAULT}"

    while [ "$#" -gt 0 ]; do
        case "$1" in
        --arch) ARCH="$2" && shift 2 ;;
        --arch=*) ARCH="${1#*=}" && shift 1 ;;
        --bin-dir) BIN_DIR="$2" && shift 2 ;;
        --bin-dir=*) BIN_DIR="${1#*=}" && shift 1 ;;
        --man-dir) MAN_DIR="$2" && shift 2 ;;
        --man-dir=*) MAN_DIR="${1#*=}" && shift 1 ;;
        --sudo) SUDO="$2" && shift 2 ;;
        --sudo=*) SUDO="${1#*=}" && shift 1 ;;
        -h | --help) usage && exit 0 ;;
        *) err "Unknown option: $1" ;;
        esac
    done
}

usage() {
    local _text_heading _text_reset
    _text_heading="$(tput bold || true 2>/dev/null)$(tput smul || true 2>/dev/null)"
    _text_reset="$(tput sgr0 || true 2>/dev/null)"

    local _arch
    _arch="$(get_architecture || true)"

    echo "\
${_text_heading}${REPO_NAME} installer${_text_reset}
${REPO_OWNER} <your-email@example.com>
https://github.com/${REPO_OWNER}/${REPO_NAME}

Fetches and installs ${REPO_NAME}. If ${REPO_NAME} is already installed, it will be updated to the latest version.

${_text_heading}Usage:${_text_reset}
  install.sh [OPTIONS]

${_text_heading}Options:${_text_reset}
      --arch     Override the architecture identified by the installer [current: ${_arch}]
      --bin-dir  Override the installation directory [default: ${BIN_DIR_DEFAULT}]
      --man-dir  Override the manpage installation directory [default: ${MAN_DIR_DEFAULT}]
      --sudo     Override the command used to elevate to root privileges [default: ${SUDO_DEFAULT}]
  -h, --help     Print help"
}

# Download the latest release asset for alman
download_alman() {
    local _arch="$1"

    if check_cmd curl; then
        _dld=curl
    elif check_cmd wget; then
        _dld=wget
    else
        need_cmd 'curl or wget'
    fi
    need_cmd grep

    local _releases_url="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"
    local _releases
    case "${_dld}" in
    curl) _releases="$(curl -sL "${_releases_url}")" ||
        err "curl: failed to download ${_releases_url}" ;;
    wget) _releases="$(wget -qO- "${_releases_url}")" ||
        err "wget: failed to download ${_releases_url}" ;;
    *) err "unsupported downloader: ${_dld}" ;;
    esac
    (echo "${_releases}" | grep -q 'API rate limit exceeded') &&
        err "you have exceeded GitHub's API rate limit. Please try again later."

    local _package_url
    _package_url="$(echo "${_releases}" | grep "browser_download_url" | cut -d '"' -f 4 | grep -- "${_arch}")" ||
        err "${REPO_NAME} has not yet been packaged for your architecture (${_arch})."

    local _ext
    case "${_package_url}" in
    *.tar.gz) _ext="tar.gz" ;;
    *.zip)     _ext="zip" ;;
    *) err "unsupported package format: ${_package_url}" ;;
    esac

    local _package="${REPO_NAME}.${_ext}"
    case "${_dld}" in
    curl) _releases="$(curl -sLo "${_package}" "${_package_url}")" || err "curl: failed to download ${_package_url}" ;;
    wget) _releases="$(wget -qO "${_package}" "${_package_url}")" || err "wget: failed to download ${_package_url}" ;;
    *) err "unsupported downloader: ${_dld}" ;;
    esac

    echo "${_package}"
}

try_sudo() {
    if "$@" >/dev/null 2>&1; then
        return 0
    fi

    need_sudo
    "${SUDO}" "$@"
}

need_sudo() {
    if ! check_cmd "${SUDO}"; then
        err "could not find the command \`${SUDO}\` needed to get permissions for install."
    fi

    if ! "${SUDO}" -v; then
        err "sudo permissions not granted, aborting installation"
    fi
}

# (Remaining helper functions unchanged...)
get_architecture() { /* ... */ }
get_bitness() { /* ... */ }
get_endianness() { /* ... */ }
is_host_amd64_elf() { /* ... */ }
check_proc() { /* ... */ }
need_cmd() { /* ... */ }
check_cmd() { /* ... */ }
ensure() { /* ... */ }
assert_nz() { /* ... */ }
err() { /* ... */ }

{ main "$@" || exit 1; }
