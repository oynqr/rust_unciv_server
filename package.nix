{ lib, rustPlatform, ... }:

rustPlatform.buildRustPackage {
  pname = "rust_unciv_server";
  version = "0.1.15";

  src = ./.;

  cargoHash = "sha256-1xhH4aX4wQueMqY7NFYxVp3I3iCZs9ORRov8p/5HcFo=";

  meta = {
    changelog = "https://github.com/oynqr/rust_unciv_server/releases";
    description = "Simple Unciv multiplayer server";
    homepage = "https://github.com/oynqr/rust_unciv_server";
    license = with lib.licenses; [
      agpl3Only
    ];
    mainProgram = "rust_unciv_server";
    maintainers = [ lib.maintainers.oynqr ];
  };
}
