-- Add down migration script here

DROP TRIGGER warehouse_stock_updated_at ON warehouse_stocks;

DROP TABLE warehouse_stocks;
