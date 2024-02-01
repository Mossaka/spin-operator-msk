- [Share Images](#share-images)
  - [To Deploy on the Cluster](#to-deploy-on-the-cluster)

# Share Images

## To Deploy on the Cluster

Build and push your image to the location specified by `IMG`:

```sh
make docker-build docker-push IMG=<some-registry>/spin-operator:tag
```

**NOTE:** This image ought to be published in the personal registry you specified. And it is required to have access to pull the image from the working environment. Make sure you have the proper permission to the registry if the above commands don’t work.
