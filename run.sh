docker run -d \
  -it \
  --name glubhs_data \
  --mount type=bind,source="$(pwd)"/src/data,target=/code/data \
  nginx:latest

docker-compose up --build

docker container stop glubhs_data
docker container rm glubhs_data

