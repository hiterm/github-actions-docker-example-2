CREATE TABLE bookshelf_user (
  id text NOT NULL PRIMARY KEY,
  created_at timestamp NOT NULL default current_timestamp,
  updated_at timestamp NOT NULL default current_timestamp
);

CREATE TABLE book_format (
  format text PRIMARY KEY
);

INSERT INTO book_format VALUES
  ('eBook'),
  ('Printed');

CREATE TABLE book_store (
  store text PRIMARY KEY
);

INSERT INTO book_store VALUES
  ('Kindle');

CREATE TABLE book (
  id uuid NOT NULL PRIMARY KEY,
  user_id text NOT NULL,
  title text NOT NULL,
  isbn text,
  read boolean NOT NULL,
  owned boolean NOT NULL,
  priority integer NOT NULL,
  format text,
  store text,
  created_at timestamp NOT NULL default current_timestamp,
  updated_at timestamp NOT NULL default current_timestamp,
  FOREIGN KEY (user_id) REFERENCES bookshelf_user(id),
  FOREIGN KEY (format) REFERENCES book_format(format) ON UPDATE CASCADE,
  FOREIGN KEY (store) REFERENCES book_store(store) ON UPDATE CASCADE
);

CREATE TABLE author (
  id uuid PRIMARY KEY,
  user_id text NOT NULL,
  name text,
  created_at timestamp NOT NULL default current_timestamp,
  updated_at timestamp NOT NULL default current_timestamp,
  FOREIGN KEY (user_id) REFERENCES bookshelf_user(id)
);

CREATE TABLE book_author (
  book_id uuid,
  author_id uuid,
  created_at timestamp NOT NULL default current_timestamp,
  updated_at timestamp NOT NULL default current_timestamp,
  PRIMARY KEY (book_id, author_id),
  FOREIGN KEY (book_id) REFERENCES book(id),
  FOREIGN KEY (author_id) REFERENCES author(id)
);