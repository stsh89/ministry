curl -X GET \
    --url https://${ASTRA_DB_ID}-${ASTRA_DB_REGION}.apps.astra.datastax.com/api/rest/v2/keyspaces/${ASTRA_DB_KEYSPACE}/ideas/4d5270ba-3ecb-4643-9221-89fdede12ebf \
    --header 'content-type: application/json' \
    --header 'accept: application/json' \
    --header "x-cassandra-token: ${ASTRA_DB_APPLICATION_TOKEN}"
