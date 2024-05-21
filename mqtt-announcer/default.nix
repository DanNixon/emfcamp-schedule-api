{
  pkgs,
  rustPlatform,
  version,
  gitRevision,
}: rec {
  emfcamp-schedule-mqtt-announcer = rustPlatform.buildRustPackage {
    pname = "emfcamp-schedule-mqtt-announcer";
    version = version;

    src = ./..;
    cargoLock.lockFile = ./../Cargo.lock;

    cargoBuildFlags = "--package emfcamp-schedule-mqtt-announcer";

    doCheck = false;
  };

  emfcamp-schedule-mqtt-announcer-container-image = pkgs.dockerTools.buildImage {
    name = "emfcamp-schedule-mqtt-announcer";
    tag = "latest";
    created = "now";

    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [pkgs.bashInteractive pkgs.coreutils];
      pathsToLink = ["/bin"];
    };

    config = {
      Entrypoint = ["${pkgs.tini}/bin/tini" "--" "${emfcamp-schedule-mqtt-announcer}/bin/emfcamp-schedule-mqtt-announcer"];
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "OBSERVABILITY_ADDRESS=0.0.0.0:9090"
      ];
    };
  };
}
