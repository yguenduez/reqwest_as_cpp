# FFI Project

Showcasing how e.g. reqwest or any other http client rust library can be linked as a static library against a C++ binary with [corrosion](https://github.com/corrosion-rs/corrosion)
and [cxx](https://cxx.rs/).  

Therefore we build those clients *without using openssl* - but with the [rustls](https://github.com/rustls/rustls) backend.

This example will just fetch the html content from *https://www.google.com* with a http GET via a *blocking reqwest client*,
which is called from a C++ executable

Idea is, that this project will be picked by the ci-runners running on MacOS, Windows and Linux and check, if those OSes support the static linking.

# How to build and run

```bash
mkdir build
cd build
cmake -S ..
cmake --build .

./main_cpp
```
