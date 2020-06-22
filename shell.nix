with import <nixpkgs> {
  overlays = [
    (import "/home/evanjs/src/nixpkgs-mozilla/rust-overlay.nix")
  ];
};

pkgs.mkShell {
  buildInputs = with pkgs.gnome3; [
    (pkgs.rustChannels.stable.rust.override { extensions = [ "rust-src" "rust-std" ]; optionalExtensions = ["rustfmt-preview" "clippy-preview" "llvm-tools-preview" "rust-analysis"]; })
    gobject-introspection
    gtk
    glib
    gdk_pixbuf
    at-spi2-core
    libconfig
    gstreamer
    pkgs.openssl
    pkgs.pkgconfig
    (pkgs.vscode-with-extensions.override {
       # When the extension is already available in the default extensions set.
       vscodeExtensions = with pkgs.vscode-extensions; [
         llvm-org.lldb-vscode
         matklad.rust-analyzer
       ];
     })   pkgs.which
   ];

 }
