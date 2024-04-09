# Rocket Notes

This small project demonstrates how to use the Rocket framework (Version 0.5) to built a rest-api.
Thematically this project implements a simple api to store a retrieve text notes.

To get information about the available endpoints, check out the `OpenApiSpec.yaml`.

## Executing tests

When running the tests with cargo make sure to only use a single thread because all tests use the same database
which nees to be in specific states for tests to succed. Using a single thread can be done with the --test-thread argument.

```Bash
cargo test -- --test-threads=1
```

## Build docker container

```Bash
# The following command can be used to build the docker image
docker build . -t rocket_notes_img

# After building the image the following command can be used to run the image
docker run -dit --name rocket_notes -p 80:80 rocket_notes_img
```

## TODO

- [ ] Add privileged (basic auth) endpoint
- [ ] Move database into a volumne in docker
- [ ] Add database migration example
