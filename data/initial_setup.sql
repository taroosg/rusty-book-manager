INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
    ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Dio',
    'dio@example.com',
    '$2b$12$GFf.eB7OpIcB3hpCr/JhoOOVPHQ0YE9oLnDA0KyHq7oGBvAFospLK',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';