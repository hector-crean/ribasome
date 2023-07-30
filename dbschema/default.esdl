# Define a module to organize the types
module default {
    # Define the User type
    type User {
        required username: str;
        required email: str {
            # Add a regular expression constraint on the email
            # constraint regexp('^([A-Za-z0-9._%-]+@[A-Za-z0-9.-]+\\.[A-Za-z]{2,})$')
        }
        required password: str {
            # Add a constraint for the password strength
            constraint min_len_value(8)
            # constraint regexp('[^A-Za-z0-9]')
        }
        # Define the default value for the updated_at
        updated_at: datetime {
            default := datetime_of_statement();
        }
        constraint exclusive on (.username)

    }

    # Define the Comment type
    type Comment {
        required title: str {
            # Add a unique constraint on the title
            constraint exclusive
        }
        required rich_text: str;
        # The published is optional
        published: bool {
            # Define a default value for published
            default := false
        }
        # Define the default values for created_at and updated_at properties
        created_at: datetime {
            default := datetime_of_statement();
        }
        updated_at: datetime {
            default := datetime_of_statement();
        }
        # Define a link to the User type with cascade deletion
        single user: User {
            on source delete delete target;
        }
    }
}
