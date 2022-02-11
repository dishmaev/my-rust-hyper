#!/bin/sh
docker run -it --rm --name test-artemis -e ARTEMIS_USERNAME=webapi -e ARTEMIS_PASSWORD=webapi -p 8161:8161 -p 61616:61616 vromero/activemq-artemis