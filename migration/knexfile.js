require("dotenv").config();
module.exports = {
  development: {
    client: "pg", // This specifies the database client
    connection: process.env.DATABASE_URL,
    migrations: {
      tableName: "knex_migrations",
      directory: "./migrations", // Path to migration files
    },
    seeds: {
      directory: "./seeds", // Path to seed files
    },
  },
};
