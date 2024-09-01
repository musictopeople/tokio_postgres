FROM postgres:latest

ENV POSTGRES_PASSWORD=changeit

EXPOSE 5432

CMD ["postgres"]
