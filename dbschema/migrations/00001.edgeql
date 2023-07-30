CREATE MIGRATION m1xao3fiqqjgsjizpoo57xdskryjorhmutuircmmrkge3bhwe4n4bq
    ONTO initial
{
  CREATE TYPE default::User {
      CREATE REQUIRED PROPERTY username: std::str;
      CREATE CONSTRAINT std::exclusive ON (.username);
      CREATE REQUIRED PROPERTY email: std::str;
      CREATE REQUIRED PROPERTY password: std::str {
          CREATE CONSTRAINT std::min_len_value(8);
      };
      CREATE PROPERTY updated_at: std::datetime {
          SET default := (std::datetime_of_statement());
      };
  };
  CREATE TYPE default::Comment {
      CREATE SINGLE LINK user: default::User {
          ON SOURCE DELETE DELETE TARGET;
      };
      CREATE PROPERTY created_at: std::datetime {
          SET default := (std::datetime_of_statement());
      };
      CREATE PROPERTY published: std::bool {
          SET default := false;
      };
      CREATE REQUIRED PROPERTY rich_text: std::str;
      CREATE REQUIRED PROPERTY title: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE PROPERTY updated_at: std::datetime {
          SET default := (std::datetime_of_statement());
      };
  };
};
