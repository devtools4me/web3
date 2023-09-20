# KAFKA

## Docker compose

```
docker-compose up -d
nc -z localhost 22181
Connection to localhost port 22181 [tcp/*] succeeded!

nc -z localhost 29092
Connection to localhost port 29092 [tcp/*] succeeded!

docker port kafka-zookeeper-1
2181/tcp -> 0.0.0.0:22181

docker port kafka-kafka-1
29092/tcp -> 0.0.0.0:29092
```

## Brew

```
brew services start zookeeper
less /opt/homebrew/var/log/zookeeper/zookeeper.log

brew services start kafka
less /opt/homebrew/var/log/kafka/kafka_output.log
```

## Docker

* Zookeeper

```
docker pull confluentinc/cp-zookeeper:5.5.1

docker run -d --net=host --name=zookeeper -e ZOOKEEPER_CLIENT_PORT=2181 -e ZOOKEEPER_TICK_TIME=2000 -e ZOOKEEPER_SYNC_LIMIT=2 confluentinc/cp-zookeeper:latest

docker logs zookeeper

docker stop zookeeper
docker rm zookeeper
```

* Kafka

```
docker pull confluentinc/cp-kafka:5.5.1

docker run -d -p 2181:2181 --net=host --name=kafka -e KAFKA_ZOOKEEPER_CONNECT=localhost:2181 -e KAFKA_ADVERTISED_LISTENERS=PLAINTEXT://localhost:9092 -e KAFKA_BROKER_ID=2 -e KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR=1 confluentinc/cp-kafka:5.5.1

docker logs kafka
```

* Kafkacat

```
docker pull confluentinc/cp-kafkacat:5.5.1

docker run --tty confluentinc/cp-kafkacat:5.5.1 kafkacat -b localhost:9092 -t new_topic -P

docker run --tty confluentinc/cp-kafkacat:5.5.1 kafkacat -b localhost:9092 -t new_topic -C

docker run --tty confluentinc/cp-kafkacat:5.5.1 kafkacat -b localhost:9092 -L

docker run --tty confluentinc/cp-kafkacat:5.5.1 kafkacat -b localhost:9092 -L -J
```