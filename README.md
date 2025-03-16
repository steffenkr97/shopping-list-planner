# shopping-list-planner

Planned functions:
- [ ] Add ingredients 
- [ ] Add recipes based on ingredients
- [ ] Add categories to recipes and ingredients
- [ ] Select desired recipes
- [ ] Export shopping list with ingredients based on selected recipes

ER-Diagram by ChatGpt
```text
+---------------------+
|   recipes           |  
+---------------------+
| id (PK)             |
| name                |
| description         |
| category_id (FK)    |
+---------------------+
        │
        ▼
+---------------------+
|  recipe_ingredients |
+--------------------+
| id (PK)           |
| recipe_id (FK)    |
| ingredient_id (FK)|
+--------------------+
        │
        ▼
+--------------------+
|   ingredients      |
+--------------------+
| id (PK)           |
| name              |
| category_id (FK)  |
+--------------------+

+--------------------+
|   weekly_plans    |
+--------------------+
| id (PK)           |
| week_start_date   |
+--------------------+
        │
        ▼
+--------------------+
|  weekly_plan_recipes |
+--------------------+
| id (PK)           |
| weekly_plan_id (FK)|
| recipe_id (FK)    |
+--------------------+

+--------------------+
|   categories      |
+--------------------+
| id (PK)           |
| name              |
+--------------------+

```