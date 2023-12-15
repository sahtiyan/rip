#!/bin/bash

# Rip derleyicisinin bulunduğu dizin
RIP_DIR=/src

# Derleyiciyi hedef bir dizine kopyala (örneğin /usr/local/bin)
cp $RIP_DIR/ripc.sh /usr/local/bin/ripc
chmod +x /usr/local/bin/ripc

# Derleyici komutunu test et
ripc --version
