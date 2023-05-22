#!/bin/bash

VERSION=${1}
wget https://github.com/shwld/devops-metrics-tools/releases/download/v${VERSION}/devops-metrics-tools-v${VERSION}-x86_64-unknown-linux-gnu.tar.xz -P /tmp
tar -Jxvf /tmp/devops-metrics-tools-v${VERSION}-x86_64-unknown-linux-gnu.tar.xz -C /tmp/.
sudo mv /tmp/devops-metrics-tools-v${VERSION}-x86_64-unknown-linux-gnu/devops-metrics-tools /usr/bin/
chmod +x /usr/bin/devops-metrics-tools
