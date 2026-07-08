{
	description = "Tiny program to alert you of new X11 apps opening";

	inputs.nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

	outputs = { self, nixpkgs }: let
		systems = [
			"x86_64-linux"
        	"aarch64-linux"
		];

		forAllSystems = f: nixpkgs.lib.genAttrs systems f;

		overlay = final: prev: {
			xwl-notifier = final.callPackage ./. { };
		};
	in {
		overlays.default = overlay;

		packages = forAllSystems (sys: let
			pkgs = import nixpkgs { system = sys; overlays = [ overlay ]; };
		in {
			default = pkgs.xwl-notifier;
		});
	};
}
