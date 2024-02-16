curl -X DELETE \
    --url "https://${ASTRA_DB_ID}-${ASTRA_DB_REGION}.apps.astra.datastax.com/api/rest/v2/keyspaces/${ASTRA_DB_KEYSPACE}/ideas/stan/2024-02-16T08:13:23.485Z/9841e34a-38dd-48bf-8d74-0ccb92ec42ad" \
    --header 'content-type: application/json' \
    --header 'accept: application/json' \
    --header "x-cassandra-token: ${ASTRA_DB_APPLICATION_TOKEN}"
