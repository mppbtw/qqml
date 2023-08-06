cmake_minimum_required(VERSION 3.0)
project(termutils)
add_library(termutils STATIC termutils.c)
install(TARGETS termutils DESTINATION .)
