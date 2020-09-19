let
  pkgs = import <nixpkgs> {};
  inherit (pkgs) stdenv;
in pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustup

    clang
    llvmPackages.libclang
    pkgconfig

    openssl
  ];
  LIBCLANG_PATH = "${pkgs.llvmPackages.libclang}/lib";
}
