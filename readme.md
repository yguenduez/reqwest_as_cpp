# Reqwest as C++

Showcasing how e.g. reqwest can be linked as a static library against a C++ binary with [corrosion](https://github.com/corrosion-rs/corrosion)
and [cxx](https://cxx.rs/).  

Therefore reqwest was build *without using openssl* - but with the [rustls](https://github.com/rustls/rustls) backend.

This example will just fetch the html content from *https://www.google.com* with a http GET via a *blocking reqwest client*,
which is called from a C++ executable

# How to build and run

```bash
mkdir build
cd build
cmake ..
make -j<threads>

./main_cpp
```
