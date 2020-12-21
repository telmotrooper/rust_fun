CREATE TABLE "employees"
(
    id SERIAL PRIMARY KEY,
    firstname VARCHAR NOT NULL,
    lastname VARCHAR NOT NULL,
    department VARCHAR NOT NULL,
    salary INT NOT NULL,
    age INT NOT NULL
);
