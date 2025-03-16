async function loadCategories() {
    const response = await fetch("/api/category/all");
    const categories = await response.json();
    const categoryList = document.getElementById("category-list");

    categoryList.innerHTML = "";
    categories.forEach(category => {
        const li = document.createElement("li");
        li.textContent = category.name;
        categoryList.appendChild(li);
    })
}


function addCategory() {
    
}

loadCategories()