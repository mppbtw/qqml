cmake_minimum_required(VERSION 3.0)
project(c_utils)
add_library(c_utils STATIC c_utils_term.c c_utils_sig.c)
install(TARGETS c_utils DESTINATION .)
