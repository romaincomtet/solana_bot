/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
exports.up = function (knex) {
  knex.schema
    .createTable("strategies", (table) => {
      table.increments("id").primary();
      table.enum("type", ["Nothing", "Sell", "Buy"]);
    })
    .createTable("discord_settings", (table) => {
      table.boolean("muted").defaultTo(false);
    })
    .createTable("users", (table) => {
      table.increments("id").primary();

      table.string("name").notNullable();
      table.timestamp("created_at").defaultTo(knex.fn.now());
      table.timestamp("updated_at").defaultTo(knex.fn.now());
    })
    .createTable("coin_settings", (table) => {
      table.increments("id").primary();
      table.integer("user_id").unsigned().notNullable();
      table.foreign("user_id").references("users.id");

      table.integer("strategy_id").unsigned().notNullable();
      table.foreign("strategy_id").references("strategies.id");

      table.integer("discord_setting_id").unsigned().notNullable();
      table.foreign("discord_setting_id").references("discord_settings.id");

      table.string("mint_address").notNullable();
      table.string("mint_symbol").notNullable();
    })
    .createTable("coins", (table) => {
      table.string("mint_address");
      table.string("mint_symbol");
      table.float("price");
      table.float("price_sol");
      table.datetime("created_at").defaultTo(knex.fn.now()).index();
      table.primary(["created_at", "mint_symbol"]);
    })
    .createTable("sells", (table) => {
      table.increments("sell_id").primary();
      table.integer("coin_setting_id").unsigned().notNullable();
      table.foreign("coin_setting_id").references("coin_settings.id");

      table.enum("config", ["OrderBook", "PercentageOf", "Grid"]);
      table.enum("Amount", ["All", "Amount", "Percentage"]);
      table.float("OrderBook");
      table.float("Grid");
      table.float("amount_value");
      table.float("percentage");
      table.float("PercentageOf");
      table.float("slippage");
    })
    .createTable("buys", (table) => {
      table.increments("sell_id").primary();
      table.integer("coin_setting_id").unsigned().notNullable();
      table.foreign("coin_setting_id").references("coin_settings.id");

      table.enum("config", ["OrderBook", "PercentageOf", "Grid"]);
      table.enum("Amount", ["All", "Amount", "Percentage"]);
      table.float("OrderBook");
      table.float("Grid");
      table.float("amount_value");
      table.float("percentage");
      table.float("PercentageOf");
      table.float("slippage");
    });
};

/**
 * @param { import("knex").Knex } knex
 * @returns { Promise<void> }
 */
exports.down = function (knex) {
  knex.schema.dropTable("users");
};
