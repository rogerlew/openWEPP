{
  description = "Reproducible openWEPP agent development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
  };

  outputs =
    { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      devTools = with pkgs; [
        bashInteractive
        cargo
        cargo-deny
        cargo-nextest
        clang
        cmake
        git
        git-lfs
        gh
        hyperfine
        jq
        mold
        nixfmt
        pkg-config
        python312
        rustc
        rustfmt
        clippy
        sccache
        shellcheck
        uv
        util-linux
      ];
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        packages = devTools;
        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [ pkgs.stdenv.cc.cc.lib ];

        shellHook = ''
          source ${./tools/dev/openwepp-env} || exit $?
          printf 'openWEPP Nix shell\n'
          printf '  target:  %s\n' "$CARGO_TARGET_DIR"
          printf '  scratch: %s\n' "$TMPDIR"
          printf '  cache:   %s\n' "$OPENWEPP_CACHE_ROOT"
        '';
      };

      checks.${system}.dev-tools =
        pkgs.runCommand "openwepp-dev-tools-check"
          {
            nativeBuildInputs = devTools;
          }
          ''
            cargo --version
            rustc --version
            cargo nextest --version
            cargo deny --version
            python3.12 --version
            uv --version
            sccache --version
            mold --version
            git-lfs --version
            shellcheck ${./tools/dev/openwepp-env} ${./tools/dev/check-host}
            touch "$out"
          '';

      formatter.${system} = pkgs.nixfmt;
    };
}
