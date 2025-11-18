fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_prost_build::configure()
    .include_file("_includes.rs")
    .build_client(cfg!(feature = "client"))
    .build_server(cfg!(feature = "server"))
    .use_arc_self(true)
    .protoc_arg("--experimental_allow_proto3_optional")
    .compile_protos(
      &[
        "share/proto/quasar/dim2.proto",
        "share/proto/quasar/dim3.proto",
        "share/proto/quasar/euler_angles.proto",
        "share/proto/quasar/latlon.proto",
        "share/proto/quasar/uuid.proto",
        "share/proto/quasar/image.proto",
        "share/proto/quasar/relay/datagrams/nav.proto",
        "share/proto/quasar/relay/datagrams/shell.proto",
        "share/proto/quasar/relay/datagrams/status.proto",

        #[cfg(feature = "grpc")]
        "share/proto/quasar/relay/services/license.proto",

        #[cfg(feature = "grpc")]
        "share/proto/quasar/relay/services/nav.proto",

        #[cfg(feature = "grpc")]
        "share/proto/quasar/relay/services/shell.proto",

        #[cfg(feature = "grpc")]
        "share/proto/quasar/relay/services/status.proto",
      ],
      &["share/proto"],
    )?;
  Ok(())
}
