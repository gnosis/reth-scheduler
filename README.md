# Rust Ethereum organization module

This modules is synchronization points between devp2p and rest of client. It organizes requests from client and messages from network, broadcasts new transaction and blocks, does warping or active/passive sync to highest block and takes care of state transition.

This is initial diagram of organizer. How development progresses it is probably going to be changed: 

![Diagram](./docs/organizer.png)

It is still in WIP stage. Interface to devp2p is is made so that it can be adapted multiple devp2p becaues they have similar functionality

Multiple managers are responsible to track current progress. For BlockManager that would be downloaded blocks/headers/receipts for snapshot manager downloaded chunks and for transaction manager is mostly here to propagate transactions.

Scheduler block is main synchronization between all sides. 

There are four sides that asynchronously affect Scheduler and influence what functionality this module needs to have:

* devp2p inbound messages is first most obvious side. We receive messages from network and need to parse them. There are two protocols Eth and Parity and multiple versions of Eth protocol 63,64 and in future 65,66.

* All send messages that we request, needs to timeout after some period of time. This means we need save all our requests and need to periodically check if messages are timed out. There should be one thread that periodically check all messages.

* We need to organize states and specify flow of execution. Usual flow of operation is: do warp -> active sync -> passive sync with background ancient block download -> passive sync. Every state requests something from network and we need to periodically check if there are new request messages that needs to be send. For simplicity I see this as thread that required messaged/actions from Managers and acts on them. Thread is periodic or can be triggered from managers. Periodic is needed to check fullness of queue and progress of inclusion of downloaded blocks, while trigger in case when we finish stage and want to trigger change. This side will contain main loop.

* NewBlock and NewTransaction events are received from deeper parts of system. If this happens we need to have ability to broadcast them to devp2p and take care what we send where.

  

  