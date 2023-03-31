cmake_minimum_required(VERSION 3.15)
project(Universal-UI)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/lib)

add_library(Universal-UI STATIC ui_application.cpp)

execute_process(COMMAND bindgen "${CMAKE_CURRENT_SOURCE_DIR}/bindings.h" "-o" "${CMAKE_CURRENT_SOURCE_DIR}/../rust-native-interface/src/intern/bindings.rs" ERROR_VARIABLE _stderr)