{ pkgs }: {
	deps = [
  pkgs.cargo-asm
  pkgs.valgrind
  pkgs.rustc
        pkgs.cargo-expand
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
        pkgs.clang
	];
}