# Project Stack Example
This project is an example of how to setup a stack containing a database and an server application.  
The stack is comprised [PostgreSQL](https://www.postgresql.org/) for the database, and a [Axum](https://docs.rs/axum/latest/axum/) application.  

## Setting Up and Running
When running locally, fill-in the `.env` file, the `POSTGRES_PASSWORD` is the password that Postgres will use, the `ALPHA_VANTAGE_API_KEY` is the key that will be used to make requests to the [AlphaVantage Stock API](https://www.alphavantage.co/), a free key can be requested [here](https://www.alphavantage.co/support/#api-key). An example of `.env` is available as `.env.example`. `POSTGRES_HOST`, `POSTGRES_USER`, and `POSTGRES_DBNAME` are exposed for further customization, but the default value of `postgres` can be used.  

Example of how the `.env` should look like, this values are for example purposes only:
```
ALPHA_VANTAGE_API_KEY="example"
POSTGRES_HOST="postgres"
POSTGRES_PORT=5432
POSTGRES_USER="postgres"
POSTGRES_PASSWORD="example"
POSTGRES_DBNAME="postgres"
```
Install [Docker](https://docs.docker.com/get-docker/) on your system.  With Docker installed, run the following command to build and run the stack:
```
docker compose up
```
After the stack is up and running, it should be possible to check the status of the containers using `docker ps -a`. The output should look something like:
```
$> docker ps -a
CONTAINER ID   IMAGE                         COMMAND                  CREATED         STATUS         PORTS                    NAMES
0649225aff88   rust_stack_example-api        "financial_data"         7 minutes ago   Up 7 seconds   0.0.0.0:8080->8000/tcp   rust_stack_example-api-1
6323931677a2   postgres:alpine               "docker-entrypoint.s…"   4 days ago      Up 7 seconds   5432/tcp                 rust_stack_example-postgres-1
```

## Initialization
On startup the table on the database is created. A background task that runs daily is also started to upsert the values of the daily times series.

## Logging
The logging level of the application can be set by adding `RUST_LOG=<LEVEL>` on the `docker-compose.yml`, in the `environment` section of the `api` service.

## Queries
The API exposes 2 endpoints: `financial_data` and `statistics`.
### ✧ `financial_data`  
Recovers the `symbol` (name of the equity), `date`, `open_price`, `close_price` and `volume`.
#### Parameters
* `symbol`: (Optional) Name of equity to recover data from.
* `start_date`: (Optional) Filters dates that are earlier than this.
* `end_date`: (Optional) Filters dates that are later than this.
* `limit`: (Optional, Default=5) Limit the number of items in the response.
* `page`: (Optional, Default=1) Get the page of number `page` for results that go over the limit.
#### Example
[http://localhost:8080/api/financial_data?start_date=2023-02-01&end_date=2023-02-28&symbol=IBM&limit=5&page=1](http://localhost:8080/api/financial_data?start_date=2023-02-01&end_date=2023-02-28&symbol=IBM&limit=5&page=1)  
**Note**: An empty response might mean that the dates are too old for when you are  

### ✧ `statistics`  
Recovers the `symbol` (name of the equity), `start_date`, `end_date`, `average_daily_open_price`, `average_daily_close_price` and `average_daily_volume`.
#### Parameters
* `symbol`: Name of equity to recover data from.
* `start_date`: Filters dates that are earlier than this.
* `end_date`: Filters dates that are later than this.
#### Example
[http://localhost:8080/api/statistics?start_date=2023-02-01&end_date=2023-03-02&symbol=IBM](http://localhost:8080/api/statistics?start_date=2023-02-01&end_date=2023-02-28&symbol=IBM)  
**Note**: An empty response might mean that the dates are too old for when you are  

## Security
For local development, the use of `.env` to set the enviroment variables of the docker compose is enough, but including it in the deployment of the production version is a security risk. Each provider has a proper way of setting enviroment variables securely, refer to the documentation of your server provider for the proper way of setting environment variables.