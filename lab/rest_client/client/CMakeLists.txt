corrosion_import_crate(MANIFEST_PATH Cargo.toml)

set(CLIENT_SO "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/libclient.dylib")

set_target_properties(client PROPERTIES LOCATION ${CMAKE_CURRENT_BINARY_DIR})

