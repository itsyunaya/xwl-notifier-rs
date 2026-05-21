{
	description = "Tiny program to alert you of new X11 apps opening";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		flake-utils.url = "github:numtide/flake-utils";
		crane.url = "github:ipetkov/crane";
	};

	outputs = { nixpkgs, flake-utils, crane, ... }:
	let
		overlay = final: prev: {
			xwl-notifier = (crane.mkLib final).buildPackage {
				src = (crane.mkLib final).cleanCargoSource ./.;
				strictDeps = true;

				buildInputs = [ final.libxcb ];
				nativeBuildInputs = [ final.pkg-config ];
			};
		};
	in
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs { inherit system; overlays = [ overlay ]; };
			in {
				packages = {
					default = pkgs.xwl-notifier;
					inherit (pkgs) xwl-notifier;
				};

				devShells.default = pkgs.mkShell {
					buildInputs = with pkgs; [ libxcb pkg-config ];
				};
			}) // {
				overlays.default = overlay;
			};
}
