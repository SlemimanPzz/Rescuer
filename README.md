# Rescuer for [Toy Ransomware](https://github.com/SlemimanPzz/Ransomware.git)

This is a rescuer written for the toy [Ransomware](https://github.com/SlemimanPzz/Ransomware.git). For it to work you need to have the RSA private key corresponding to the public RSA key use for encrypting the AES256 key that was use by the ransomware, the RSA private key should be named decrypt.txt. After running it will decrypt all the encrypted files.

## For building 
- Download [Rust](https://www.rust-lang.org/tools/install) 
- Do `cargo build --release` get the executable, if you are not on a windows system add the `--target=x86_64-pc-windows-gnu` flag
- The `Rescuer.exe` will be located in `taget/release/Ransomware.exe` or in `taget/debug/Ransomware.exe` if you didn't use the `--release` flag.

## While running

It need the private key to desencrypt the AES256 use for encrypting all files. It should be name `decrypt.txt` and also dond delete the key use to encryp the files