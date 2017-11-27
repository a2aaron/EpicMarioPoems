#!/bin/sh

date >> ~/logs/mario-epics.log
./target/release/mario-epics >> ~/logs/mario-epics.log 2>&1

