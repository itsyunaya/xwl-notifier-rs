{ pkgs, lib, pkg-config }:
	pkgs.rustPlatform.buildRustPackage {
		pname = "xwl-notifier";
		version = "0.2.0";
		cargoLock.lockFile = ./Cargo.lock;
		src = pkgs.lib.cleanSource ./.;

		nativeBuildInputs = [ pkg-config ];
		buildInputs = [ pkgs.libxcb ];

		meta = {
			description = "Tiny program to alert you of new X11 apps opening";
			homepage = "https://github.com/itsyunaya/xwl-notifier-rs";
			license = lib.licenses.gpl3;
			platforms = [ "x86_64-linux" "aarch64-linux" ];
		};
	}
