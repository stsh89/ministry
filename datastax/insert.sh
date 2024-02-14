curl --request POST \
    --url https://${ASTRA_DB_ID}-${ASTRA_DB_REGION}.apps.astra.datastax.com/api/rest/v2/keyspaces/${ASTRA_DB_KEYSPACE}/ideas \
    --header 'content-type: application/json' \
    --header "x-cassandra-token: ${ASTRA_DB_APPLICATION_TOKEN}" \
    --data '{"id":"4d5270ba-3ecb-4643-9221-89fdede12ebf","inserted_at":"2019-01-10T09:48:31.020Z","thought":"Be master of your life!"}'
