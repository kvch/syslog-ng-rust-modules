if (CMAKE_BUILD_TYPE STREQUAL "Release")
  set(CARGO_BUILD ${CARGO_EXECUTABLE} build --release)
  set(TARGET_DIR "target/release")
else()
  set(CARGO_BUILD ${CARGO_EXECUTABLE} build)
  set(TARGET_DIR "target/debug")
endif()

function(add_lib_crate CRATE_NAME)
  string(REPLACE "-" "_" NORMALIZED_CRATE_NAME ${CRATE_NAME})
  set(LIBNAME "lib${NORMALIZED_CRATE_NAME}.so")
  add_custom_target(${CRATE_NAME} ALL ${CARGO_BUILD} WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
  set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES ${CMAKE_CURRENT_SOURCE_DIR}/target)
  string(TOUPPER ${NORMALIZED_CRATE_NAME} CRATE_NAME_UC)

  set(${CRATE_NAME_UC}_LIB_TARGET ${CMAKE_CURRENT_SOURCE_DIR}/${TARGET_DIR}/${LIBNAME} PARENT_SCOPE)
endfunction()

function(add_cargo_bin_target TARGET_NAME)
  add_custom_target(${TARGET_NAME} ALL ${CARGO_BUILD} WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
  set_directory_properties(PROPERTIES ADDITIONAL_MAKE_CLEAN_FILES ${CMAKE_CURRENT_SOURCE_DIR}/target)
  string(TOUPPER ${TARGET_NAME} TARGET_NAME_UC)

  set(${TARGET_NAME_UC}_PATH ${CMAKE_CURRENT_SOURCE_DIR}/${TARGET_DIR}/${TARGET_NAME} PARENT_SCOPE)
endfunction()
