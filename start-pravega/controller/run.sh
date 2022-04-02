export ZK_IP=10.247.97.51

if [[ "$1" == start ]]
then
    docker-compose up -d
elif [[ "$1" == stop ]]
then
    docker-compose down
fi