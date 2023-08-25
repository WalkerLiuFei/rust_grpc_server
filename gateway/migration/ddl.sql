CREATE DATABASE IF NOT EXISTS gateway;
USE gateway;
CREATE TABLE uri_grpc_mapping
(
    id           INT AUTO_INCREMENT PRIMARY KEY,
    http_uri     VARCHAR(255) NOT NULL,
    grpc_service VARCHAR(255) NOT NULL,
    grpc_method  VARCHAR(255) NOT NULL,
    created_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY         uni_idx_uri(http_uri),
    UNIQUE KEY         uni_idx_grpc(grpc_service,grpc_method)
);

