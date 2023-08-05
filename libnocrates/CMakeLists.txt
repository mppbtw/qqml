cmake_minimum_required(VERSION 3.0)
project(libnocrates)
add_library(nocrates STATIC utils.c)
install(TARGETS nocrates DESTINATION .)
