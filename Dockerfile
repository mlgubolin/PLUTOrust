FROM rust:latest
ADD . /code
CMD ["mkdir","./data"]
CMD [ "docker volume create","~/data:~/data" ]
WORKDIR /code
#RUN pip install -r requirements.txt
CMD ["cargo", "build"]
CMD ["cargo", "run"]