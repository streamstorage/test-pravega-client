export PRAVEGA_LTS=/home/charlie/test
export BOOKIE_IP=10.247.97.51
export ZK_IP=10.247.97.51
export CONTROLLER_IP=10.247.97.51
export SEGMENTSTORE_IP=10.247.97.19 # your segmentstore host ip

if [[ "$1" == start ]]
then
    docker-compose up -d
elif [[ "$1" == stop ]]
then
    docker-compose down
fi
