# Rocket Notes

This small project demonstrates how to use the Rocket framework (Version 0.5) to built a rest-api.
Thematically this project implements a simple api to store a retrieve text notes.

To get information about the available endpoints, check out the `OpenApiSpec.yaml`.

## Build docker container

```Bash
# The following command can be used to build the docker image
docker build . -t rocket_notes_img

# After building the image the following command can be used to run the image
docker run -dit --name rocket_notes -p 80:80 rocket_notes_img
```

## Todo

- [ ] Add a persistent database to store the notes in
