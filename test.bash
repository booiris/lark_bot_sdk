#!/bin/bash

features=("acs" "ai" "admin" "application" "approval" "attendance" "baike" "bitable" "bot" "calendar" "chat" "contact" "core_hr" "drive" "ehr" "event" "file" "helpdesk" "hire" "human_auth" "jssdk" "lingo" "mail" "message" "mina" "minutes" "okr" "passport" "performance" "personal_settings" "search" "task" "tenant" "vc" "verification")

rm -rf test_result
mkdir test_result

parallel=15
if [ $# -eq 1 ]; then
    parallel=$1
fi
start=$(date +%s)

for ((i = 0; i < ${#features[@]}; i += parallel)); do
    subset=()
    for ((j = i; j < i + parallel && j < ${#features[@]}; j++)); do
        subset+=(${features[j]})
    done
    subset=$(
        IFS=,
        echo "${subset[*]}"
    )
    echo "Testing $subset"
    # cargo test --features=$subset,test-util --no-default-features --package lark_bot_sdk --lib -- api::gen >test_result/$subset.txt    
    # cargo test --features=$subset,test-util --no-default-features --package lark_bot_sdk --lib -- api::gen
    if ! cargo test --features=$subset,test-util --no-default-features --package lark_bot_sdk --lib -- api::gen; then
        end=$(date +%s)
        runtime=$((end - start))
        echo "Error occurred while testing $subset at $(date)"
        echo "Total execution time: $runtime seconds"
        exit 1
    fi
done

end=$(date +%s)

runtime=$((end - start))

echo "Total execution time: $runtime seconds"
