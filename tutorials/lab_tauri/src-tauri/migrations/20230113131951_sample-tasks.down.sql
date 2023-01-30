WITH bmstudent_id as (
    SELECT
        id
    FROM
        users
    WHERE
        login = 'bmstudent'
),
alordash_id as (
    SELECT
        id
    FROM
        users
    WHERE
        login = 'alordash'
)
DELETE FROM
    tasks
WHERE
    (description, created_by) IN (
        (
            'Exams preparation',
            (
                SELECT
                    *
                FROM
                    bmstudent_id
            )
        ),
        (
            'Holidays',
            (
                SELECT
                    *
                FROM
                    bmstudent_id
            )
        ),
        (
            'Summer exams',
            (
                SELECT
                    *
                FROM
                    alordash_id
            )
        )
    );