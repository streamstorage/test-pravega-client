## Deployment
To start pravega in Distributed Mode with docker compose.

1. start common
2. start segment_store
3. start controller

To have docker service started and docker-compose tool installed as prerequisites. Then go to each folder and run following command.

```
sudo bash run.sh start
```

## Reference
1. [Completed docker compose file](https://github.com/pravega/pravega/blob/master/docker/compose/docker-compose.yml)

2. [Upgrade Guide](https://github.com/pravega/pravega-operator/blob/master/doc/upgrade-cluster.md#upgrade-guide)
