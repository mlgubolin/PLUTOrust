FROM rust:latest
ADD . /code
WORKDIR /code
#RUN pip install -r requirements.txt
CMD ["cargo", "build"]