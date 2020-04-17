# fy

fy, abbreviation of fanyi, is a translation cli tool written by rust.

## build and install on Linux and Mac X OS

steps:

1. install rust (https://www.rust-lang.org/tools/install)
2. [optional] setup mirror of crates.io to speed up building process in China.
    ```
    mkdir -p ~/.cargo
    echo > ~/.cargo/config <<EOF
    [source.crates-io]
    replace-with = 'ustc'
    
    [source.ustc]
    registry = "https://mirrors.ustc.edu.cn/crates.io-index"
    EOF
    ```
3. bash ./install_on_local.sh
4. chmod +x ~/.cargo/bin/fy
5. execute `fy --help` and enjoy.

![fy-help](./fy-help.png)

<hr>

![fy-example](./fy-example.png)