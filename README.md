# Rust API Playground

Learning the language by building a simple API

The following calls require local CRDB to be running with a users table

Example calls:

### register

```shell
http POST localhost:9090/register email=rob@robgodwin.com password=12345
```

### login
 This will return the JWT needed for subsequent calls
```shell
http POST localhost:9090/login email=rob@robgodwin.com password=12345
```

### update

```shell
http -A bearer -a j.w.t POST localhost:9090/update email=new@email.com
```

### delete

```shell
http -A bearer -a j.w.t POST localhost:9090/delete
```

### user_profile

```shell
http -A bearer -a j.w.t GET localhost:9090/user_profile
```