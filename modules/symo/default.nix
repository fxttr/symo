flake: { config, lib, pkgs, ... }:

let
  inherit (lib) filterAttrs types mkEnableOption mkOption;
  inherit (flake.packages.${pkgs.stdenv.hostPlatform.system}) default;

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
        default = default;
        description = ''
          The symo package to use with the service
        '';
      };
    };
  };

  config = lib.mkIf cfg.enable {
    systemd.user.services.symo = {
      Unit = {
        Description = "symo";
        PartOf = [ cfg.systemdTarget ];
        After = [ cfg.systemdTarget ];
      };

      Service = {
        Type = "exec";
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

      Install = {
        WantedBy = [ cfg.systemdTarget ];
      };
    };
  };
}
