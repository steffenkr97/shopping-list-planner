document.addEventListener("DOMContentLoaded", () => {
    loadCategories();

    document.getElementById("category-form").addEventListener("submit", async function (event) {
        event.preventDefault();
        const categoryName = document.getElementById("category-name").value;

        const response = await fetch("/api/category", {
            method: "POST",
            headers: {"Content-Type": "application/json"},
            body: JSON.stringify({name: categoryName})
        });

        if (response.ok) {
            loadCategories();
            this.reset();
        } else {
            alert("Fehler beim Hinzufügen der Kategorie!");
        }
    });

});

async function loadCategories() {
    const response = await fetch("/api/category/all");
    const categories = await response.json();
    const categoryList = document.getElementById("category-list");
    categoryList.innerHTML = "";

    categories.forEach(category => {
        const li = document.createElement("li");
        li.textContent = category.name;
        categoryList.appendChild(li);
    });
}
