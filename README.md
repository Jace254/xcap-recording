# XCap Recorder

## Development

> [!IMPORTANT]
> I'm developing in a windows environment.<br>
> The command line commands here must be run in Git Bash not command prompt or you will get syntax errors.

### Dependencies

#### C++ build environment

Download [MSVC](https://visualstudio.microsoft.com/) and install.
Select `Windows` as Developer machine OS and check `C++`, then download Visual Studio Community version and install. The installation may take a while.

#### Rust develop environment

Download [rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) and run it as administrator to install `rust`.

#### vcpkg

Go to the folder you want to clone vcpkg and use [Git Bash](https://git-scm.com/download/win) to run the following commands, download `vcpkg`, install 64-bit version of `libvpx`, `libyuv` and `opus`.
If you don't have `Git` installed, get `Git` [here](https://git-scm.com/download/win).

```bash
git clone https://github.com/microsoft/vcpkg
cd vcpkg
vcpkg/bootstrap-vcpkg.bat
export VCPKG_ROOT=$PWD/vcpkg
vcpkg/vcpkg install libvpx:x64-windows-static
```

Add System environment variable `VCPKG_ROOT`=`<path>\vcpkg`. The `<path>` should be the location you choose above to clone `vcpkg`.

### Build

Run `cargo build --release`.

### Run Dev Mode

Run:

```PowerShell
 $env:VPX_LIB_DIR=<VCPKG_ROOT/vpx_lib_path>; $env:VPX_INCLUDE_DIR=<VCPKG_ROOT/vpx_include_path>; $env:VPX_VERSION=<found_in_control_file_from_vpx_static>; cargo run
```
