_: {
  perSystem =
    {
      pkgs,
      config,
      ...
    }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        programs = {
          rustfmt = {
            enable = true;
          };
          nixfmt = {
            enable = true;
          };
          statix = {
            enable = true;
          };
        };
      };
    };
}
