cmake_minimum_required(VERSION 3.5)
project(libsobol)
set(CMAKE_CXX_STANDARD 14)
add_library(sobol STATIC libsobol.cpp sobol.cpp)
install(TARGETS sobol DESTINATION ${CMAKE_INSTALL_PREFIX})
