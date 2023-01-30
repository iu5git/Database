INSERT INTO
    tasks (description, done, due_time, created_by)
VALUES
    (
        'Exams preparation',
        TRUE,
        current_timestamp + '5 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'bmstudent'
        )
    ),
    (
        'Holidays',
        FALSE,
        current_timestamp + '20 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'bmstudent'
        )
    ),
    (
        'Summer exams',
        FALSE,
        current_timestamp + '100 days',
        (
            SELECT
                id
            FROM
                users
            WHERE
                login = 'alordash'
        )
    );