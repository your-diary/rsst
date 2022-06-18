#!/usr/bin/env bash

if [[ $# == 2 && $1 == '--interval' ]]; then
    i=$2
else
    echo 'Usage: ./cron.sh --interval <minutes>'
    exit 1
fi

set -e
((i * 60))
log_file='./conf/log.txt'
touch "${log_file}"
discord_webhook_url="$(jq --raw-output '.triggers | .discord | .webhook_url' './conf/config.json')"
[[ "${discord_webhook_url}" == 'https://discord.com/'* ]]
set +e

cargo build --release

echo "Hereafter, all outputs to stdout and stderr are redirected to the log file [ ${log_file} ]."
exec >> "${log_file}" 2>&1

while true; do
    echo "--------------- $(date) ---------------"
    ./target/release/rsst
    if [[ $? != 0 ]]; then
        curl --silent -X POST "${discord_webhook_url}" -H 'Content-Type: application/json' -d "$(echo '{"wait": true, "content": "rsst: Failed. See the log."}' | jq --rawfile log <(tail "${log_file}") '.content += "\n\n" + $log')"
    fi
    echo
    for ((j = 0; j < i; ++j)); do
        restart='./conf/.restart'
        if [[ -f "${restart}" ]]; then
            rm "${restart}"
            break
        fi
        sleep 60
    done
done

