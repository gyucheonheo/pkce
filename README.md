# PKCE

Code challenge & code verifier generator

## Install
``` bash
$>cargo install --path .
```

## Usage

``` bash
$>pkce
```

``` bash
$>pkce --verifier <verifier>
```

``` bash
$>pkce --length <length> --fpath <file_path>
```

If `<length>` is not specified, its default value is 64.
If `<file_path>` is not specified, it will use `abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-._~` as `charset` to generate `code_verifier`.

