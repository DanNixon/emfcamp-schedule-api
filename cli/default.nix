{
  pkgs,
  rustPlatform,
  version,
  gitRevision,
}: rec {
  emfcamp-schedule-cli = rustPlatform.buildRustPackage {
    pname = "emfcamp-schedule-cli";
    version = version;

    src = ./..;
    cargoLock.lockFile = ./../Cargo.lock;

    cargoBuildFlags = "--package emfcamp-schedule-cli";

    doCheck = false;
  };

  emfcamp-schedule-cli-container-image = pkgs.dockerTools.buildImage {
    name = "emfcamp-schedule-cli";
    tag = "latest";
    created = "now";

    config = {
      Entrypoint = ["${pkgs.tini}/bin/tini" "--" "${emfcamp-schedule-cli}/bin/emfcamp-schedule-cli"];
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
      ];
    };
  };
}
