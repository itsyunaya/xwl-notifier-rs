{ pkgs, lib, pkg-config }: let
	manifest = builtins.fromTOML (builtins.readFile ../Cargo.toml);
	inherit (manifest.package) name version;
in
	pkgs.rustPlatform.buildRustPackage {
		pname = name;
		inherit version;
		cargoLock.lockFile = ../Cargo.lock;
		src = pkgs.lib.cleanSource ../.;

		nativeBuildInputs = [ pkg-config ];
		buildInputs = [ pkgs.libxcb ];

		meta = {
			description = "Tiny program to alert you of new X11 apps opening";
			homepage = "https://github.com/itsyunaya/xwl-notifier-rs";
			maintainers = [ lib.maintainers.itsyunaya ];
			license = lib.licenses.gpl3Plus;
			platforms = [ "x86_64-linux" "aarch64-linux" ];
		};
	}
