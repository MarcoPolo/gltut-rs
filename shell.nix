{pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [glfw] ++ (with pkgs.darwin.apple_sdk.frameworks; [Carbon Cocoa Kernel]);
}
