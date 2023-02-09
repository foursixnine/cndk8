set(CMAKE_CXX_STANDARD 14)


set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_INCLUDE_CURRENT_DIR ON)



find_package(Qt6Widgets)
find_package(VulkanHeaders)

set(SOURCE main_window.cpp main_window.hpp main.cpp)
add_executable(gui ${SOURCE})

get_target_property(CLIENT_SO client LOCATION)
target_link_libraries(gui Qt6::Widgets)
target_link_libraries(gui ${CLIENT_SO}/libclient.dylib)


add_dependencies(gui client)
