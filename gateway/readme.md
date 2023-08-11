create table by execute the `ddl.sql` file.

run to generate entity 
```bash
sea-orm-cli generate entity \
    -u mysql://root:password@localhost:3306/bakeries_db \
    -o src/entities
```
modify change to your own database connection string.

