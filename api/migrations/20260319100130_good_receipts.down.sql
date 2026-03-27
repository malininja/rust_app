-- Add down migration script here

DROP TRIGGER goods_receipt_items_updated_at ON goods_receipt_items;

DROP TABLE goods_receipt_items;

DROP TRIGGER goods_receipt_heads_updated_at ON goods_receipt_heads;

DROP TABLE goods_receipt_heads;