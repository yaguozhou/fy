# fy  ![Continuous integration](https://github.com/abstractsuperman/fy/workflows/Continuous%20integration/badge.svg)

fy, abbreviation of fanyi, is an english-chinese translation cli tool based
 on youdao api, and written in rust.

## build and install on Linux and Mac OS X

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
3. git clone https://github.com/abstractsuperman/fy.git
4. bash install_on_local.sh
5. chmod +x ~/.cargo/bin/fy
6. execute `fy --help` and enjoy.

![fy-help](./fy-help.png)

<hr>

![fy-example](./fy-example.png)
