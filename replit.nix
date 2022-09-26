{ pkgs }: {
    deps = [
        # Compiler
        pkgs.sudo
                           pkgs.rustc
        # Project / library manager
        pkgs.cargo
        # Formats your code
        pkgs.rustfmt
        # Lints your code
        pkgs.clippy

        # Allows Replit package manager to work
        # pkgs.cargo-edit

        # Required for Raylib
        pkgs.raylib
    ];

    env = {
        RAYLIB_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.raylib
        ];
    };
}
