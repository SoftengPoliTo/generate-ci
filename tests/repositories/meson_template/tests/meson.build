# Create a new executable object to test the library
exe = executable(
  'test-meson_template', # Executable name
  'meson_template.c', # Tests source file
  include_directories: incs, # Directories to be included when building the executable
  dependencies: meson_template_dep # Libraries to be linked at the executable
)

# Create a test that run all tests contained in the executable produced above
test(
  'test_name', # Test name
  exe # Executable object
)
