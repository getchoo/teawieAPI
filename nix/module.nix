self:
{
  config,
  lib,
  pkgs,
  ...
}:
let
  cfg = config.services.teawie-api;

  inherit (lib)
    getExe
    literalExpression
    mkEnableOption
    mkIf
    mkOption
    mkPackageOption
    types
    ;

  inherit (pkgs.stdenv.hostPlatform) system;
in
{
  options.services.teawie-api = {
    enable = mkEnableOption "teawieAPI";
    package = mkPackageOption (self.packages.${system} or (throw "${system} is not supported!")
    ) "teawie-api" { };

    listen = {
      address = mkOption {
        type = types.str;
        default = "127.0.0.1";
        description = "IP address to listen on";
        example = "::";
      };

      port = mkOption {
        type = types.port;
        default = 7777;
        description = "TCP port that will be used to accept client connections";
        example = 6969;
      };
    };

    environmentFile = mkOption {
      type = types.nullOr types.path;
      default = null;
      description = ''
        Environment file as defined in {manpage}`systemd.exec(5)`
      '';
      example = literalExpression ''
        "/run/agenix.d/1/teawieAPI"
      '';
    };
  };

  config = mkIf cfg.enable {
    systemd.services.teawie-api = {
      enable = true;
      wantedBy = [ "multi-user.target" ];
      after = [ "network.target" ];

      script = ''
        ${getExe cfg.package}
      '';

      environment = {
        LISTEN_ADDR = lib.concatMapStringsSep ":" toString [
          cfg.listen.address
          cfg.listen.port
        ];
      };

      serviceConfig = {
        Type = "simple";
        Restart = "on-failure";

        EnvironmentFile = mkIf (cfg.environmentFile != null) cfg.environmentFile;

        DynamicUser = true;

        # Hardening options
        PrivateDevices = true;
        PrivateIPC = true;
        PrivateUsers = true;

        ProtectClock = true;
        ProtectControlGroups = true;
        ProtectHome = true;
        ProtectHostname = true;
        ProtectKernelLogs = true;
        ProtectKernelModules = true;
        ProtectKernelTunables = true;
        ProtectProc = "invisible";
        ProcSubset = "pid";

        # We don't need UNIX sockets or anything
        RestrictAddressFamilies = [
          "AF_INET"
          "AF_INET6"
        ];
        RestrictNamespaces = true;
        RestrictRealtime = true;

        LockPersonality = true;
        MemoryDenyWriteExecute = true;

        SystemCallFilter = [ "@system-service" ];
        SystemCallArchitectures = "native";

        UMask = "0077";
      };
    };
  };
}
