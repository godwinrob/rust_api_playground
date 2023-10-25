# Rust API Playground

Learning the language by building a simple API

Example calls:

### register

```shell
http POST localhost:9090/register email=rob@robgodwin.com password=12345
```

### login

```shell
http POST localhost:9090/login email=rob@robgodwin.com password=12345
```

### update

```shell
http -A bearer -a eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InJvYkByb2Jnb2R3aW4uY29tIiwiZXhwIjoxNjk4Mjc4NTcyfQ.I4onAtUZUSTwVETKX0ik3mDaFTCS_p2qge16TDvxFaA POST localhost:9090/update email=new@email.com
```

### delete

```shell
http -A bearer -a eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InJvYkByb2Jnb2R3aW4uY29tIiwiZXhwIjoxNjk4Mjc4NTcyfQ.I4onAtUZUSTwVETKX0ik3mDaFTCS_p2qge16TDvxFaA POST localhost:9090/delete
```

### user_profile

```shell
http -A bearer -a eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJlbWFpbCI6InJvYkByb2Jnb2R3aW4uY29tIiwiZXhwIjoxNjk4Mjc4NTcyfQ.I4onAtUZUSTwVETKX0ik3mDaFTCS_p2qge16TDvxFaA GET localhost:9090/user_profile
```