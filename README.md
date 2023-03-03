# Pucciniales

Pucciniales is a open-source firebase alternative aiming to support as many features as the original using only fully open-source projects with non restrictive licenses ( see [Licenses](#licenses)).

## Tech stack

- Server: Rust levraging **[actic-web](https://github.com/actix/actix-web)**
- Databases:
  - Cache: Redis
  - SQL: PostgreSQL
  - NoSQL: ArangoDB

### Explenation

- actix-web
  - Why?
    - It is one of the best established options while being sufficiently fast. This increases the amount of people capable of contributing to the project, increases the chances of vulnerabilities being reported and eases development.
- Redis
  - Why?
    - Well established, very mature and performs well.
- PostgreSQL
  - Why?
    - Well established, very mature and performs well.
  - Why not CockroachDB?
    - I would be willing to trade the huge establishment of PostgreSQL against the horizontal scalability of CockroachDB but their licensing is simply far too restrictive to offer it as the native SQL database.
- ArangoDB
  - Why?
    - Mature and is useful accross the board for key-value, document, semi-relational and graph databases.
  - Why not MongoDB?
    - Not quite as varied as Arango and I am too unsure about possible ramifications of the license for people who want to use Pucciniales.

## Licenses

- **Libraries**
  - **[actic-web](https://github.com/actix/actix-web)**: [Apache-2.0](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0)) or [MIT](https://tldrlegal.com/license/mit-license)
- **Databases**
  - **[Redis](https://redis.io)**: [BSD 3-clause](https://tldrlegal.com/license/bsd-3-clause-license-(revised))
  - **[PostgreSQL](https://www.postgresql.org)**: [FOSSAPostgreSQL License](https://tldrlegal.com/license/postgresql-license-(postgresql))
  - **[ArangoDB](https://www.arangodb.com)**: [Apache-2.0](https://tldrlegal.com/license/apache-license-2.0-(apache-2.0))
