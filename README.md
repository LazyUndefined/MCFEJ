**MCFEJ - MCFunction Editor for Java edition**
---
# LICENSE
    - MCFEJ is distributed under Apache-2.0.
      Please see LICENSE or <https://www.apache.org/licenses/LICENSE-2.0.html> .
    
# Introduction
    - MCFEJ は GUI の Minecraft コマンドエディタです。
    - 動作には Cargo, Tcl/Tk が必要です。
    - 開発は超スローペースになりますが、どうぞよろしくお願いします。
# Build from source
    - まずコマンドラインに cargo --version と入力し、Cargo がインストールされていることを確認してください。
      バージョンが出力されたらインストールされています。
    - コマンドラインで MCFEJ をカレントディレクトリにし、cargo build --release と入力してください。
      target/release に mcfej と言う名前の実行ファイルが存在するはずです。
# Acknowledgements
    MCFEJ depends on these softwares. Please see COPYRIGHT-NOTICE.
    - rstk <https://crates.io/crates/rstk>
    - serde <https://crates.io/crates/serde>
