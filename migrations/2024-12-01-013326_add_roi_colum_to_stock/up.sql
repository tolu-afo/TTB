-- Your SQL goes here
ALTER TABLE stocks
ADD COLUMN roi_percentage DECIMAL(10, 2) NOT NULL DEFAULT 0.1;
