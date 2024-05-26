self: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.teawieapi;

  inherit
    (lib)
    getExe
    literalExpression
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    types
    ;

  inherit (pkgs.stdenv.hostPlatform) system;
in {
  options.services.teawieapi = {
    enable = mkEnableOption "teawieapi";
    package = mkPackageOption (
      self.packages.${system} or (throw "${system} is not supported!")
    ) "teawieapi" {};

    listen = {
      address = mkOption {
        type = types.str;
        default = "127.0.0.1";
        example = "::";
        description = "IP address to listen on";
      };

      port = mkOption {
        type = types.port;
        default = 7777;
        example = "6969";
        description = "TCP port that will be used to accept client connections";
      };
    };

    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      example = literalExpression ''
        "/run/agenix.d/1/teawieAPI"
      '';
      description = ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services."teawieapi" = {
      enable = true;
      wantedBy = ["multi-user.target"];
      after = ["network.target"];

      script = ''
        ${getExe cfg.package}
      '';

      environment = {
        LISTEN_ADDR = "${cfg.listen.address}:${toString cfg.listen.port}";
      };

      serviceConfig = {
        Type = "simple";
        Restart = "always";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        # hardening
        DynamicUser = true;
        NoNewPrivileges = true;
        PrivateDevices = true;
        PrivateTmp = true;
        PrivateUsers = true;
        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectSystem = "strict";
        RestrictNamespaces = "uts ipc pid user cgroup";
        RestrictSUIDSGID = true;
        Umask = "0007";
      };
    };
  };
}
