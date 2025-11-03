https://neon.tech/postgresql/postgresql-cheat-sheet

# Copilot

Creating a database in PostgreSQL and then using its URL involves a few steps. Here’s a concise guide to get you started:

### Step 1: Install PostgreSQL

If you haven't already, install PostgreSQL on your machine. You can download it from the [official PostgreSQL website](https://www.postgresql.org/download/).

### Step 2: Start PostgreSQL Server

After installation, make sure the PostgreSQL server is running. You can usually start it from your command line or terminal:

```
pg_ctl -D /usr/local/var/postgres start
```

### Step 3: Connect to PostgreSQL

Open the PostgreSQL interactive terminal, `psql`, with the following command:

```
psql -U postgres
```

Here, `-U postgres` specifies connecting as the `postgres` user.

### Step 4: Create a User (Optional)

You might want to create a specific user for your database:

```
CREATE USER myuser WITH ENCRYPTED PASSWORD 'mypassword';
```

Replace `myuser` and `mypassword` with your preferred username and password.



### Step 5: Create a Database

Once you’re inside the `psql` environment, create a new database using:

```
CREATE DATABASE my_database OWNER new_user;
```
note: If you use the command below, you can't create databse schemas when migrating with sqlx migrate run command.
```
CREATE DATABASE mydatabase;
```

Replace `mydatabase` with your desired database name.

### Step 6: Grant Privileges

Grant privileges to your user for the newly created database:

```
GRANT ALL PRIVILEGES ON DATABASE mydatabase TO myuser;
```

### Step 7: Construct Database URL

Now, construct the database URL for connecting. The URL format for PostgreSQL is:

```
postgresql://[username]:[password]@[hostname]:[port]/[database]
```

For example, it might look like this:

```
postgresql://myuser:mypassword@localhost:5432/mydatabase
```

Replace `myuser`, `mypassword`, `localhost`, `5432`, and `mydatabase` with your actual username, password, hostname, port, and database name.

### Using the Database URL

To use this URL in your application (for example, in a Python app using SQLAlchemy):

```python
from sqlalchemy import create_engine
engine = create_engine('postgresql://myuser:mypassword@localhost:5432/mydatabase')
```

### Final Check

Make sure your PostgreSQL server is running, and you should be able to connect to your database using the URL you constructed.

If you run into any issues or have any specific questions, feel free to ask! I'm here to help.

### Change password

```
ALTER USER myuser WITH PASSWORD 'newpassword';
```
