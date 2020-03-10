let
	moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
	nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in
	with nixpkgs;
stdenv.mkDerivation {
	name = "rust-env";
	buildInputs = [
		nixpkgs.latest.rustChannels.stable.rust
		rustracer
		rustfmt
		rustPlatform.rustcSrc
		rls
		cargo
		pkgconfig
	];
	RUST_BACKTRACE 	= 1;
	RUST_SRC_PATH 	= "${latest.rustChannels.stable.rust-src}/lib/rustlib/src/rust/src";
}
