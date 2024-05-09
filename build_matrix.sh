#!/bin/bash
set -e

cd testbin

host_target="x86_64-unknown-linux-gnu"
# Note: Using another target that can run on this computer for convenience,
# both for running and so that I don't need a sysroot.
other_target="i686-unknown-linux-gnu"

if [ "$(rustc -vV | sed -n 's|host: ||p')" != "$host_target" ]
then
    echo "Please set host_target=$(rustc -vV | sed -n 's|host: ||p') and other_target to something not that"
fi

# Table header
echo "With a host of $host_target"

echo "| | target_applies_to_host=true | target_applies_to_host=false |"
echo "|-|-|-|"

for target in "" "$host_target" "$other_target"
do
    if [ "" = "$target" ]
    then
        target_flags=""
        table_label="no --target flag"
    else
        target_flags="--target ${target}"
        table_label=$target_flags
    fi

    echo -n "| ${table_label} |"

    for target_applies_to_host in "true" "false"
    do
        cmd="CARGO_TARGET_APPLIES_TO_HOST='$target_applies_to_host' RUSTFLAGS='--cfg flag' cargo build --quiet -Z target-applies-to-host $target_flags"
        if [ "$target" = "$other_target" ]
        then
            bash -c "$cmd"
            output=$(./target/${target}/debug/testbin | sed 's_$_<br/>_g' | tr -d '\n')
        else
            bash -c "$cmd"
            output=$(./target/${target}/debug/testbin | sed 's_$_<br/>_g' | tr -d '\n')
        fi
        echo -n " ${output} |"
    done
    echo
done