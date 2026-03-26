-- Add down migration script here

DROP TRIGGER warehouse_stock_updated_at ON warehouse_stock;

DROP TABLE warehouse_stock;
