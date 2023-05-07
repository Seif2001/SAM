cmake_minimum_required(VERSION 3.0)
project(systemcall C)
add_library(systemcall STATIC systemcall.c)
install(TARGETS systemcall DESTINATION .)