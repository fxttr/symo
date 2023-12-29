flake: { config, lib, pkgs, ... }:

let
  inherit (lib) filterAttrs types mkEnableOption mkOption;
  inherit (flake.packages.${pkgs.stdenv.hostPlatform.system}) symo;

  cfg = config.services.symo;
in 
{
  options = {
    services.symo = {
      enable = mkEnableOption ''
        A system monitor for unix-like operating systems 
      '';

      memory = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Monitor memory consumption
        '';
      };

      network = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Show network information
        '';
      };

      battery = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Show battery level
        '';
      };

      volume = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Show the pipewire volume
        '';
      };

      date = mkOption {
        type = types.bool;
        default = true;
        description = ''
          Show the current time and date
        '';
      };

      package = mkOption {
        type = types.package;
        default = symo;
        description = ''
          The symo package to use with the service
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.services.symo = {
      description = "symo";
      documentation = [ "https://github.com/fxttr/symo" ];

      after = [ "multi-user.target" ];

      serviceConfig = {
        Restart = "always";
        ExecStart = "${lib.getBin cfg.package}/target/release/symo";

        # Hardening
        CapabilityBoundingSet = [ "AF_NETLINK" "AF_INET" "AF_INET6" ];
        DeviceAllow = [ "/dev/stdin r" ];
        DevicePolicy = "strict";
        IPAddressAllow = "localhost";
        LockPersonality = true;

        # MemoryDenyWriteExecute = true;
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
        ReadOnlyPaths = [ "/" ];
        RemoveIPC = true;
        RestrictAddressFamilies = [ "AF_NETLINK" "AF_INET" "AF_INET6" ];
        RestrictNamespaces = true;
        RestrictRealtime = true;
        RestrictSUIDSGID = true;
        SystemCallArchitectures = "native";
        SystemCallFilter = [ "@system-service" "~@privileged" "~@resources" "@pkey" ];
        UMask = "0027";
      };

      preStart = ''
        install -d -m750 ${config.services.symo.dataDir}/Config
        rm -f "$installedConfigFile" && install -m640 ${configFile} "$installedConfigFile"
      '';
    };
  };
}