# C files contained in the directory
cli_src = files('meson_template.c')

# Create a new executable
meson_template_cli = executable(
    'meson_template', # Executable name
    cli_src, # Executable files
    install: true, # Install the executable in some default filesystem positions
    include_directories: incs, # Directories to be included when building the executable
    dependencies: meson_template_dep # Libraries to be linked at the executable
)
