curl -X GET \
    --url https://${ASTRA_DB_ID}-${ASTRA_DB_REGION}.apps.astra.datastax.com/api/rest/v2/keyspaces/${ASTRA_DB_KEYSPACE}/ideas \
    --header 'content-type: application/json' \
    --header 'accept: application/json' \
    --header "x-cassandra-token: ${ASTRA_DB_APPLICATION_TOKEN}" \
    --data-urlencode 'where={
        "id": {"$eq": "197d0dd0-caa3-11ee-a9ad-554cf9c1cc56"}
    }'
