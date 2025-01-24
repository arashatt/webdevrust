Sure, I can help you with that! Here’s a step-by-step guide to create a database and a user in MySQL:

1. **Log in to MySQL:**

   ```sql
   mysql -u root -p
   ```

2. **Create a new database:**

   ```sql
   CREATE DATABASE my_database;
   ```

3. **Create a new user:**

   ```sql
   CREATE USER 'new_user'@'localhost' IDENTIFIED BY 'password';
   ```

4. **Grant the user access to the database:**

   ```sql
   GRANT ALL PRIVILEGES ON my_database.* TO 'new_user'@'localhost';
   ```

5. **Flush the privileges to make sure that they are saved and available in the current MySQL session:**

   ```sql
   FLUSH PRIVILEGES;
   ```

6. **Verify that the user has the privileges:**
   ```sql
   SHOW GRANTS FOR 'new_user'@'localhost';
   ```

Replace `my_database` with the name of your database, `new_user` with your desired username, and `password` with a secure password.

If you encounter any issues or have specific requirements, feel free to ask!

### Change Password

```
ALTER USER 'username'@'localhost' IDENTIFIED BY 'new_password';
```
### Instructions
```
SELECT * FROM emp
```
this will be equivalent to the code:
```rust
conn.execute(sqlx::query("SELECT * FROM emp")).await?;
// or
sqlx::query("DELETE FROM table").execute(&pool).await?;
```

the sqlx build query and executes on the connection you have.


### Migrations manually deleted

what would you do when your migrations are manually deleted?
the `sqlx migration run` argues that there was a migrated already ran, but it cannot find it anymore.
your database is corrupt when the migration is lost. so what is the solution?

you just need to connect to databae and delete the migration row in `_sqlx_migrations` row.
