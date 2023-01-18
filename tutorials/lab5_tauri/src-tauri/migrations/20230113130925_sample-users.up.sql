INSERT INTO
    users (login, password)
VALUES
    (
        'alordash',
        crypt('12345', gen_salt('bf'))
    ),
    (
        'bmstudent',
        crypt('2025graduate', gen_salt('bf'))
    );