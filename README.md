# Rescuer for Toy [Ransomware](https://github.com/SlemimanPzz/Ransomware.git)

This is a rescuer written for the toy [Ransomware](https://github.com/SlemimanPzz/Ransomware.git). For it to work, you need to have the RSA private key corresponding to the public RSA key used for encrypting the AES256 key that was used by the ransomware. The RSA private key should be named `decrypt.txt`. After running it, it will decrypt all the encrypted files.

## For Building
- Download [Rust](https://www.rust-lang.org/tools/install)
- Run `cargo build --release` to get the executable. If you're not on a Windows system, add the `--target=x86_64-pc-windows-gnu` flag.
- The `Rescuer.exe` will be located in `target/release/Rescuer.exe` or `target/debug/Rescuer.exe` if you didn't use the `--release` flag.

## While Running
It needs the private key to decrypt the AES256 key used for encrypting all files. The private key should be named `decrypt.txt`. Also, don't delete the key used to encrypt the files.
