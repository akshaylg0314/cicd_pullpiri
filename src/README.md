<!--
SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.

SPDX-License-Identifier: Apache-2.0
-->

# PULLPIRI Structures

(under construction)

For `Pullpiri` build, try following in the project root folder

```bash
# in pullpiri folder
make image
```

## server

only 1 node

1. apiserver
1. policymanager
1. monitoringserver
1. etcd

## player

only 1 node (server + player can be located in 1 node)

1. filtergateway
1. actioncontroller
1. statemanager

## agent

all nodes

1. nodeagent

## common

common resources like gRPC proto, pullpiri artifact, ...

## tools

Additional tools will be added to aid development.
