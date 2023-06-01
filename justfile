set positional-arguments
get-version:
    #!/usr/bin/env bash
    source makedeb/PKGBUILD
    echo "${pkgver}"

update-versions:
    #!/usr/bin/env bash
    set -euo pipefail
    version="$(just get-version)"
    sed -i "s|^version = .*|version = \"${version}\"|" Cargo.toml
