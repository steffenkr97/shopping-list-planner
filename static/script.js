document.addEventListener("DOMContentLoaded", () => {
    loadCategories();
    loadIngredients();

    document
        .getElementById("category-form")
        .addEventListener("submit", async function (event) {
            event.preventDefault();
            await addCategory();
        });

    document
        .getElementById("ingredient-form")
        .addEventListener("submit", async function (event) {
            event.preventDefault();
            await addIngredient();
        })

});

async function addCategory() {
    const categoryName = document.getElementById("category-name").value;

    const response = await fetch("/api/category", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({name: categoryName})
    });

    if (response.ok) {
        await loadCategories();
    } else {
        console.log(response);
        alert("Fehler beim Hinzufügen der Kategorie!");
    }
}

async function loadCategories() {
    const response = await fetch("/api/category/all");
    const categories = await response.json();
    const categoryList = document.getElementById("category-list");
    const categorySelect = document.getElementById("category-select");

    categoryList.innerHTML = "";
    categorySelect.innerHTML = "";

    categories.forEach(category => {

        const li = document.createElement("li");
        li.textContent = category.name;
        categoryList.appendChild(li);

        const option = document.createElement("option");
        option.value = category.id;
        option.textContent = category.name;
        categorySelect.appendChild(option);
    });
}

async function addIngredient() {

    const respone = await fetch("/api/ingredient", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({
            name: document.getElementById("ingredient-name").value,
            description: document.getElementById("ingredient-description").value,
            category_id: parseInt(document.getElementById("category-select").value),
        })
    })

    if (respone.ok) {
        loadIngredients()
    } else {
        console.log(respone);
        alert("Fehler beim hinzufügen von Zutaten")
    }

}

async function loadIngredients() {
    const response = await fetch("/api/ingredient/all");
    const ingredients = await response.json();
    const ingredientList = document.getElementById("ingredient-list");
    ingredientList.innerHTML = "";

    ingredients.forEach(category => {
        const li = document.createElement("li");
        li.textContent = category.name;
        ingredientList.appendChild(li);
    });
}
