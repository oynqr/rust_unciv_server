{ lib, rustPlatform, ... }:

rustPlatform.buildRustPackage {
  pname = "rust_unciv_server";
  version = "0.1.15";

  src = ./.;

  cargoHash = "sha256-43oQErkjx82TuWI4Kh/9WR/RwMZX0K2g8QxpYD0jvvM=";

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
