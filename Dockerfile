FROM rust:latest
ADD . /code
WORKDIR /code
#RUN pip install -r requirements.txt
CMD ["mkdir","./data"]
CMD ["cargo", "build"]
CMD ["cargo", "run"]