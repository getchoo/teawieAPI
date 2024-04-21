self: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.teawieapi;
  inherit (pkgs.stdenv.hostPlatform) system;
in {
  options.services.teawieapi = {
    enable = lib.mkEnableOption "teawieapi";
    package = lib.mkPackageOption (
      self.packages.${system} or (throw "${system} is not supported!")
    ) "teawieapi" {};

    listen = {
      address = lib.mkOption {
        type = lib.types.str;
        default = "127.0.0.1";
        example = "::";
        description = "IP address to listen on";
      };

      port = lib.mkOption {
        type = lib.types.port;
        default = 7777;
        example = "6969";
        description = "TCP port that will be used to accept client connections";
      };
    };

    environmentFile = lib.mkOption {
      description = ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      type = lib.types.nullOr lib.types.path;
      default = null;
      example = lib.literalExpression ''
        "/run/agenix.d/1/teawieAPI"
      '';
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services."teawieapi" = {
      enable = true;
      wantedBy = ["multi-user.target"];
      after = ["network.target"];

      script = ''
        ${lib.getExe cfg.package}
      '';

      environment = {
        LISTEN_ADDR = "${cfg.listen.address}:${toString cfg.listen.port}";
      };

      serviceConfig = {
        Type = "simple";
        Restart = "always";

        EnvironmentFile = lib.mkIf (cfg.environmentFile != null) cfg.environmentFile;

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
