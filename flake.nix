{
	description = "Tiny program to alert you of new X11 apps opening";

	inputs.nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";

	outputs = { nixpkgs, ... }: let
		systems = [
			"x86_64-linux"
			"aarch64-linux"
		];

		forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
	in {
		packages = forAllSystems (pkgs: let
			default = pkgs.callPackage ./nix/package.nix {};
		in {
			inherit default;
			xwl-notifier = default;
		});

		devShells = forAllSystems (pkgs: {
			default = pkgs.callPackage ./nix/shell.nix {};
		});
	};
}
