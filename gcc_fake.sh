#!/bin/sh
gcc -Xlinker --defsym=FLAG_LINKER=1 $@
