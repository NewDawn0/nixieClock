{
  description = "CLI clock display in nixie tubes";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    utils = {
      url = "github:NewDawn0/nixUtils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, utils, ... }@inputs:
    let
      mkPkgs = system:
        import nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };
    in {
      overlays.default = final: prev: {
        nixie-clock = self.packages.${prev.system}.default;
      };
      packages = utils.lib.eachSystem { inherit mkPkgs; } (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "nixie-clock";
          version = "1.0.0";
          src = ./.;
          useFetchCargoVendor = true;
          cargoHash = "sha256-T+oRN10Nf39AoPHZav0PZVKVU9zBjBpYGNz8PINkGYg=";
          meta = {
            description = "CLI clock display in nixie tubes";
            longDescription = ''
              A unique command-line clock that displays the current time using Nixie tube-style digits.
              This charming design adds a vintage touch to your terminal while providing an accurate clock.
            '';
            homepage = "https://github.com/NewDawn0/nixieClock";
            license = pkgs.lib.licenses.mit;
            maintainers = with pkgs.lib.maintainers; [ NewDawn0 ];
            platforms = pkgs.lib.platforms.all;
          };
        };

      });
    };
}
