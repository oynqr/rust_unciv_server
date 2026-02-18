{ lib, rustPlatform, ... }:

rustPlatform.buildRustPackage {
  pname = "rust_unciv_server";
  version = "0.1.16";

  src = ./.;

  cargoHash = "sha256-8TzwITgrjaQQoKJbLC/Igez03Sfb8x5ZJ0RxRRJq8jE=";

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
