FROM python:3.8-slim-buster

ENV SECRET_KEY='secret'
ENV DEBUG=False
ENV DATABASE_URI='sqlite:///db.sqlite'
ENV HOST=0.0.0.0
ENV PORT=5000
ENV GLOBAL_PREFIX=/api/v1

WORKDIR /python-docker

COPY requirements.txt .
RUN pip3 install -r requirements.txt

COPY main.py .
COPY src ./src

EXPOSE 5000

CMD [ "python3", "main.py"]
