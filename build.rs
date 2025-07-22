fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = tonic_build::Config::new();
  config.protoc_arg("--experimental_allow_proto3_optional");

  tonic_build::configure()
    .build_client(true)
    .build_server(true)
    .use_arc_self(true)
    .compile_protos_with_config(
      config,
      &[
        "share/proto/quasar/dim2.proto",
        "share/proto/quasar/dim3.proto",
        "share/proto/quasar/euler_angles.proto",
        "share/proto/quasar/latlon.proto",
        "share/proto/quasar/uuid.proto",
        "share/proto/quasar/image.proto",
        "share/proto/quasar/relay/datagrams/nav.proto",
        "share/proto/quasar/relay/datagrams/shell.proto",
        "share/proto/quasar/relay/services/license.proto",
        "share/proto/quasar/relay/services/nav.proto",
        "share/proto/quasar/relay/services/shell.proto",
      ],
      &["share/proto"],
    )?;
  Ok(())
}
