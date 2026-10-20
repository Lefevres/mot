{ lib, fetchFromGitHub, rustPlatform, }:

rustPlatform.buildRustPackage {
  pname = "mot";
  version = "0.1.0";

  src = fetchFromGitHub {
    owner = "Lefevres";
    repo = "mot";
    rev = "main";
    hash = lib.fakeHash;
  };

  cargoHash = lib.fakeHash; 

}
