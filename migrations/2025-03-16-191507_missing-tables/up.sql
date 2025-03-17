-- Your SQL goes here
CREATE TABLE recipes
(
    id          INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    description TEXT,
    category_id INTEGER,
    FOREIGN KEY (category_id) REFERENCES categories (id)
);

CREATE TABLE recipe_ingredients
(
    id            INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    recipe_id     INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    FOREIGN KEY (recipe_id) REFERENCES recipes (id),
    FOREIGN KEY (ingredient_id) REFERENCES ingredients (id),
    UNIQUE (recipe_id, ingredient_id)
);

CREATE TABLE weekly_plans
(
    id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    week_start_date DATE NOT NULL
);

CREATE TABLE weekly_plan_recipes
(
    id             INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    weekly_plan_id INTEGER NOT NULL,
    recipe_id      INTEGER NOT NULL,
    FOREIGN KEY (weekly_plan_id) REFERENCES weekly_plans (id),
    FOREIGN KEY (recipe_id) REFERENCES recipes (id),
    UNIQUE (weekly_plan_id, recipe_id)
);
