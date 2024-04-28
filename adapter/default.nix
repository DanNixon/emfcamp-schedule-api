{
  pkgs,
  rustPlatform,
  version,
  gitRevision,
}: rec {
  emfcamp-schedule-api-adapter = rustPlatform.buildRustPackage {
    pname = "emfcamp-schedule-api-adapter";
    version = version;

    src = ./..;
    cargoLock.lockFile = ./../Cargo.lock;

    cargoBuildFlags = "--package emfcamp-schedule-api-adapter";

    doCheck = false;
  };

  emfcamp-schedule-api-adapter-container-image = pkgs.dockerTools.buildImage {
    name = "emfcamp-schedule-api-adapter";
    tag = "latest";
    created = "now";

    copyToRoot = pkgs.buildEnv {
      name = "image-root";
      paths = [pkgs.bashInteractive pkgs.coreutils];
      pathsToLink = ["/bin"];
    };

    config = {
      Entrypoint = ["${pkgs.tini}/bin/tini" "--" "${emfcamp-schedule-api-adapter}/bin/emfcamp-schedule-api-adapter"];
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "API_ADDRESS=0.0.0.0:8000"
        "OBSERVABILITY_ADDRESS=0.0.0.0:9090"
      ];
    };
  };
}
