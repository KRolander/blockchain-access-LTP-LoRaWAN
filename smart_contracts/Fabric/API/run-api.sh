
ENV_DAL=`echo $DISCOVERY_AS_LOCALHOST`

echo "ENV_DAL:"$DISCOVERY_AS_LOCALHOST

if [ "$ENV_DAL" != "true" ]
then
	export DISCOVERY_AS_LOCALHOST=false
fi

echo "DISCOVERY_AS_LOCALHOST="$DISCOVERY_AS_LOCALHOST

echo "run api..."

go run api.go
