# C files contained in the directory
cli_src = files('meson_project.c')

# Create a new executable
meson_project_cli = executable(
    'meson_project', # Executable name
    cli_src, # Executable files
    install: true, # Install the executable in some default filesystem positions
    include_directories: incs, # Directories to be included when building the executable
    dependencies: meson_project_dep # Libraries to be linked at the executable
)
