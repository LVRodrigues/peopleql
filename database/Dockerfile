FROM postgres:latest

ENV POSTGRES_PASSWORD 'postgres'
ENV POSTGRES_DB 'guess-who'
ENV POSTGRES_USER 'guess-admin'
ENV POSTGRES_PASSWORD 'guess-pass'

COPY \
    setup/001-users-docker.sql \
    setup/003-database-roles.sql \
    setup/004-domains.sql \
    setup/005-schemas.sql \
    setup/006-tables.sql \
    setup/007-views.sql \
    setup/008-sequences.sql \
    setup/009-functions.sql \
    setup/010-roles.sql \
    setup/011-populate.sql \
    /docker-entrypoint-initdb.d/

EXPOSE 5432